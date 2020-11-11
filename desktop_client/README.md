# CliView Desktop

[![Python](https://www.python.org/static/community_logos/python-powered-w-200x80.png)](https://www.python.org/)

#### Command line mode:
*The errors mentioned below only concern use-case errors and not server errors.*
1. Casting a file to device, it will start to play the video immediately. Multiple links can be provided but only the first one will be used:
    ```
    python main.py --cast url
    ```
    
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```missing 1 required positional argument: 'link'``` | Missing url |

1. Adds links to the playlist queue:
    ```
    python main.py --mcast url1 url2 ... urln
    ```
     **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```missing 1 required positional argument: 'links'``` | Missing url(s) |
    
1. Casting the next link in queue, skipping the current video:
    ```
    python main.py --next
    ```
     **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | - | - |
    
1. Scrolling in the video positive value is forward negative value is backward. The given value is automatically multiplied by 30:
    ```
    python main.py --scroll value
    ```
    
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```missing 1 required positional argument: 'value'``` | Missing value |
    
#### GUI mode:
##### Possible errors: