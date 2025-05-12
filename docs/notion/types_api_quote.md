Quote

Quote block objects contain the following information within the quote
property:

  -----------------------------------------------------------------------
  Field                   Type                    Description
  ----------------------- ----------------------- -----------------------
  rich_text               array of rich text      The rich text displayed
                          objects                 in the quote block.

  color                   string (enum)           The color of the block.
                                                  Possible values are:
                                                  -
                                                  
                                                  "blue"
                                                  -
                                                  
                                                  "blue_background"
                                                  -
                                                  
                                                  "brown"
                                                  -
                                                  
                                                  "brown_background"
                                                  -
                                                  
                                                  "default"
                                                  -
                                                  
                                                  "gray"
                                                  -
                                                  
                                                  "gray_background"
                                                  -
                                                  
                                                  "green"
                                                  -
                                                  
                                                  "green_background"
                                                  -
                                                  
                                                  "orange"
                                                  -
                                                  
                                                  "orange_background"
                                                  -
                                                  
                                                  "yellow"
                                                  -
                                                  
                                                  "green"
                                                  -
                                                  
                                                  "pink"
                                                  -
                                                  
                                                  "pink_background"
                                                  -
                                                  
                                                  "purple"
                                                  -
                                                  
                                                  "purple_background"
                                                  -
                                                  
                                                  "red"
                                                  -
                                                  
                                                  "red_background"
                                                  -
                                                  
                                                  "yellow_background"

  children                array of block objects  The nested child
                                                  blocks, if any, of the
                                                  quote block.
  -----------------------------------------------------------------------

Example Quote block

    {
        //...other keys excluded
        "type": "quote",
       //...other keys excluded
       "quote": {
        "rich_text": [{
          "type": "text",
          "text": {
            "content": "To be or not to be...",
            "link": null
          },
            //...other keys excluded
        }],
        //...other keys excluded
        "color": "default"
       }
    }
