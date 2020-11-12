# CliView Desktop

[![Python](https://www.python.org/static/community_logos/python-powered-w-200x80.png)](https://www.python.org/)

#### Installation:
 With .yaml file:
    Use the .yaml file in ```CliView\desktop_client\desktop\desktop\Installation``` folder it includes everything needed to run the application and the server too.

#### Command line mode:
*The errors mentioned below only concern use-case errors and not server errors.*
1. Basically this is multicast with one url. Multiple urls can be provided but only the first one will be used [untested]:
    ```
    python main.py --cast url
    ```
    
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```missing 1 required positional argument: 'link'``` | Missing url |

1. Adds url to the playlist queue  [untested]:
    ```
    python main.py --mcast url1 url2 ... urln
    ```
     **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```missing 1 required positional argument: 'links'``` | Missing url(s) |
    
1. Skips the current video:
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