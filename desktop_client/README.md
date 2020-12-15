# CliView Desktop

[![Python](https://www.python.org/static/community_logos/python-powered-w-200x80.png)](https://www.python.org/)

![Desktop app](https://github.com/gyaur/CliView/workflows/Desktop%20app%20(Python)%20continuous%20integration/badge.svg)

#### Installation:
 With .yaml file:
    Use the .yaml file in ```CliView\desktop_client\desktop\desktop\Installation``` folder it includes everything needed to run the application and the server too.

#### Command line mode:
*The errors mentioned below only concern use-case errors and not server errors.*
1. It will change the settings.json file ip and port number:
    ```
    python main.py --set ip port
    ```
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```cmd_set() missing 1 required positional argument: 'settings'``` | Missing values |
    |```Missing argument for --set command. Use this command like : --set ip port ```| Ip address or port number missing. |
    
1. It will start the video if it was stopped earlier or do nothing:
    ```
    python main.py --start
    ```
    
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | - | - |
    
1. It will stop the video if it was started earlier or do nothing:
    ```
    python main.py --stop
    ```
    
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | - | - |

1. Basically this is multicast with one url. Multiple urls can be provided but only the first one will be used. The "" are needed because some cases the url gets interpreted in pieces. The local file sharing was tested on the mock server, the url still does not work. (Might be windows error):
    ```
    python main.py --cast "url"
    ```
    
    **Possible errors:**
    | Error | Problem |
    | ------ | ------ |
    | ```missing 1 required positional argument: 'link'``` | Missing url |

1. Adds url to the playlist queue  [untested]:
    ```
    python main.py --mcast "url1" "url2" ... "urln"
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