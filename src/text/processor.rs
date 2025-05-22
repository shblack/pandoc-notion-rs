use crate::text::{TextFormat, TextProcessor, TextProcessingError};
use pandoc_types::definition::Pandoc;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;

/// TextProcessor implementation using the Pandoc executable
pub struct PandocProcessor {
    /// Path to the pandoc executable
    pandoc_path: String,
    /// Default options to apply to all Pandoc commands
    default_options: Vec<String>,
}

impl PandocProcessor {
    /// Create a new PandocProcessor with default configuration
    pub fn new() -> Self {
        Self {
            pandoc_path: "pandoc".to_string(),
            default_options: Vec::new(),
        }
    }
    
    /// Create a new PandocProcessor with a custom path to the pandoc executable
    pub fn with_pandoc_path<S: Into<String>>(pandoc_path: S) -> Self {
        Self {
            pandoc_path: pandoc_path.into(),
            default_options: Vec::new(),
        }
    }
    
    /// Add a default option to be used in all conversions
    pub fn with_default_option<S: Into<String>>(mut self, option: S) -> Self {
        self.default_options.push(option.into());
        self
    }
    
    /// Get pandoc format string, using commonmark_x for markdown
    fn get_pandoc_format(&self, format: TextFormat) -> &'static str {
        match format {
            TextFormat::Markdown => "commonmark_x",
            TextFormat::CommonMark => "commonmark",
            TextFormat::GithubMarkdown => "gfm",
            TextFormat::PlainText => "plain",
            TextFormat::Html => "html",
            TextFormat::Latex => "latex",
            TextFormat::Rst => "rst",
            TextFormat::Org => "org",
            TextFormat::Custom(fmt) => fmt,
        }
    }
    
    /// Check if Pandoc is available and get its version
    pub fn check_pandoc_availability(&self) -> Result<String, TextProcessingError> {
        let output = self.create_command(&["--version"])
            .output()?;
        
        if output.status.success() {
            let version_info = String::from_utf8(output.stdout)?;
            // Extract just the first line which contains the version
            let first_line = version_info.lines().next()
                .ok_or_else(|| TextProcessingError::Other("Could not parse Pandoc version".to_string()))?;
            Ok(first_line.to_string())
        } else {
            Err(TextProcessingError::PandocError(
                "Pandoc is not available on this system".to_string()
            ))
        }
    }
    
    /// Get a list of supported output formats from Pandoc
    pub fn get_supported_formats(&self) -> Result<Vec<String>, TextProcessingError> {
        let output = self.create_command(&["--list-output-formats"])
            .output()?;
        
        if output.status.success() {
            let formats_str = String::from_utf8(output.stdout)?;
            let formats = formats_str.lines().map(String::from).collect();
            Ok(formats)
        } else {
            Err(TextProcessingError::PandocError(
                "Failed to retrieve supported formats".to_string()
            ))
        }
    }
    
    // Helper method to create a Command with default options
    fn create_command(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new(&self.pandoc_path);
        
        // Add default options
        for opt in &self.default_options {
            cmd.arg(opt);
        }
        
        // Add specific args
        cmd.args(args);
        
        cmd
    }
}

impl TextProcessor for PandocProcessor {
    fn text_to_ast(&self, text: &str, format: TextFormat) -> Result<Pandoc, TextProcessingError> {
        // Run pandoc to convert text to JSON AST
        let mut child = self.create_command(&[
            "-f", self.get_pandoc_format(format),
            "-t", "json"
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
        
        // Write text to pandoc's stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        
        // Get the output
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(TextProcessingError::PandocError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        // Parse JSON output
        let json_str = String::from_utf8(output.stdout)?;
        let ast: Pandoc = serde_json::from_str(&json_str)?;
        
        Ok(ast)
    }
    
    fn ast_to_text(&self, ast: &Pandoc, format: TextFormat) -> Result<String, TextProcessingError> {
        // Serialize AST to JSON
        let json_str = serde_json::to_string(ast)?;
        
        // Run pandoc to convert JSON to the desired format
        let mut child = self.create_command(&[
            "-f", "json",
            "-t", self.get_pandoc_format(format)
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
        
        // Write JSON to pandoc's stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(json_str.as_bytes())?;
        }
        
        // Get the output
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(TextProcessingError::PandocError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        let result = String::from_utf8(output.stdout)?;
        Ok(result)
    }
    
    fn convert_text(
        &self,
        text: &str,
        from_format: TextFormat,
        to_format: TextFormat
    ) -> Result<String, TextProcessingError> {
        // Create pandoc command with input and output formats
        let mut child = self.create_command(&[
            "-f", self.get_pandoc_format(from_format),
            "-t", self.get_pandoc_format(to_format)
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
        
        // Write input text to pandoc's stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        
        // Get the output
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(TextProcessingError::PandocError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        let result = String::from_utf8(output.stdout)?;
        Ok(result)
    }
    
    fn file_to_ast_with_format<P: AsRef<Path>>(
        &self,
        path: P,
        format: TextFormat
    ) -> Result<Pandoc, TextProcessingError> {
        let path_str = path.as_ref().to_str()
            .ok_or_else(|| TextProcessingError::Other("Invalid path".to_string()))?;
        
        // Run pandoc to convert the file to JSON AST
        let output = self.create_command(&[
            path_str,
            "-f", self.get_pandoc_format(format),
            "-t", "json"
        ])
        .output()?;
        
        if !output.status.success() {
            return Err(TextProcessingError::PandocError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        // Parse JSON output
        let json_str = String::from_utf8(output.stdout)?;
        let ast: Pandoc = serde_json::from_str(&json_str)?;
        
        Ok(ast)
    }
    
    fn ast_to_file_with_format<P: AsRef<Path>>(
        &self,
        ast: &Pandoc,
        path: P,
        format: TextFormat
    ) -> Result<(), TextProcessingError> {
        // Serialize AST to JSON
        let json_str = serde_json::to_string(ast)?;
        
        let path_str = path.as_ref().to_str()
            .ok_or_else(|| TextProcessingError::Other("Invalid path".to_string()))?;
        
        // Run pandoc to convert JSON to the output file
        let mut child = self.create_command(&[
            "-f", "json",
            "-t", self.get_pandoc_format(format),
            "-o", path_str
        ])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
        
        // Write JSON to pandoc's stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(json_str.as_bytes())?;
        }
        
        // Wait for completion
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(TextProcessingError::PandocError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        Ok(())
    }
    
    fn convert_file<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Q
    ) -> Result<(), TextProcessingError> {
        // Determine formats from file extensions
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();
        
        let from_format = input_path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(TextFormat::from_extension)
            .unwrap_or(TextFormat::Markdown);
            
        let to_format = output_path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(TextFormat::from_extension)
            .unwrap_or(TextFormat::Markdown);
            
        self.convert_file_with_format(input_path, output_path, from_format, to_format)
    }
    
    fn convert_file_with_format<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Q,
        from_format: TextFormat,
        to_format: TextFormat
    ) -> Result<(), TextProcessingError> {
        let input_path_str = input_path.as_ref().to_str()
            .ok_or_else(|| TextProcessingError::Other("Invalid input path".to_string()))?;
            
        let output_path_str = output_path.as_ref().to_str()
            .ok_or_else(|| TextProcessingError::Other("Invalid output path".to_string()))?;
        
        // Run pandoc to convert directly from input to output file
        let output = self.create_command(&[
            input_path_str,
            "-f", self.get_pandoc_format(from_format),
            "-t", self.get_pandoc_format(to_format),
            "-o", output_path_str
        ])
        .output()?;
        
        if !output.status.success() {
            return Err(TextProcessingError::PandocError(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        Ok(())
    }
}

// Convenience function to create a new processor
pub fn create_processor() -> PandocProcessor {
    PandocProcessor::new()
}