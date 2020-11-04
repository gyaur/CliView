'''
Constants which are used in the application 
'''

#Constants for the flask server:
IP      =  "http://127.0.0.1:"
PORT    =  "5000/"
ADDRESS =  IP + PORT

CAST    = "stream"                  #All the routes on the server
MCAST   = "queue"
NEXT    = "skip"
SCROLL  = "seek"
ROUTES  = {
           CAST  : ADDRESS+CAST,
           MCAST : ADDRESS+MCAST,
           NEXT  : ADDRESS+NEXT,
           SCROLL: ADDRESS+SCROLL
          }

