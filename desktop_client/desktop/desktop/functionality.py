import requests
import tkinter as tk
from tkinter   import messagebox, ttk, filedialog
from constants import ROUTES, CAST, MCAST, NEXT, SCROLL

#This is the backend...
class Functionality:
    def __init__(self) :
        self.links     = []
        self.responder = Responder()
        self.commands  = {"--cast"  : self.cmd_cast,
                          "--prev"  : self.previous,
                          "--next"  : self.next,
                          "--mcast" : self.cmd_mcast,
                          "--scroll": self.cmd_scroll,
                         }

    def error(self, msg:str) :
        messagebox.showerror(title="Error", message=msg)
    
    def upload(self, tkList:tk.Listbox, link:str):
        tkList.insert("end", link)

    #On the GUI cast the first link and add to queue the rest 
    def cast(self, tkList:tk.Listbox) :
       
       link = tkList.get(0)
       self.cmd_cast([link])
       self.cmd_mcast(tkList.get(1, "end"))
    
    #It will cast the video at once
    def cmd_cast(self, link:list) :
        self.responder.post(ROUTES[CAST], {"url": link[0]})

    
    #This will add all links to a queue
    def cmd_mcast(self, links:list) :
        [ self.responder.post(ROUTES[MCAST], {"url": link}) for link in links]

    def previous(self):
        pass

    def next(self):
        self.responder.post(ROUTES[NEXT])

    def save_links(self, tkList:tk.Listbox):
        with filedialog.asksaveasfile(mode='w', defaultextension=".txt") as file:
            data  = tkList.get(0, "end")
            file.write("\n".join(data))
    
    #Auto multiply the value with 30.
    def cmd_scroll(self,value :int):
        self.responder.post(ROUTES[SCROLL], {"seek": value*30})
    
    def scroll_video(self, command:str, btn:ttk.Button = None) :
        def bbb() : self.responder.post(ROUTES[SCROLL], {"seek": -5*30})
        def bb()  : self.responder.post(ROUTES[SCROLL], {"seek": -1*30})
        def ff()  : self.responder.post(ROUTES[SCROLL], {"seek":  1*30})
        def fff() : self.responder.post(ROUTES[SCROLL], {"seek":  5*30})
        def start(btn:ttk.Button):
            if btn["text"] == "START" :
                print("Start the video")
                btn["text"] = "STOP"
                return
            print("Stop the video")
            btn["text"] = "START"

        commands = {"bbb" : bbb, "bb" : bb, "ff":ff, "fff":fff, "start":start}
        if btn : 
            commands[command](btn)
            return
        commands[command]()

#This is the communication between the server and the application
class Responder:
    def __init__(self):
        pass
    
    def post(self, address:str, msg=None) -> int:
        r = requests.post(address, params=msg)
        return r.status_code


    
