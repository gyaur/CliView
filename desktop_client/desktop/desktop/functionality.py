import requests
import tkinter as tk
from tkinter   import messagebox, ttk, filedialog
from constants import CAST, MCAST, NEXT, SCROLL, VOLUME

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
                          "--set"   : self.cmd_set,
                          "--volume": self.cmd_volume
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
        self.responder.post(CAST, {"url": link[0]})

    
    #This will add all links to a queue
    def cmd_mcast(self, links:list) :
        [ self.responder.post(MCAST, {"url": link}) for link in links]

    
    def cmd_volume(self, value:list) :
        err = self.responder.post(VOLUME, {"volume": value[0]}) 
        if err != 200 :
            raise CustomError(f"An error occurred, server response : {err}")

    def set_volume(self, volume : str):
        v = int(float(volume))
        self.cmd_volume([v])

    def previous(self):
        pass

    def next(self):
        self.responder.post(NEXT)

    def save_links(self, tkList:tk.Listbox):
        with filedialog.asksaveasfile(mode='w', defaultextension=("text files","*.txt")) as file:
            data  = tkList.get(0, "end")
            file.write("\n".join(data))

    def load_links(self, tkList:tk.Listbox) :
        tkList.delete(0, "end")

        filename = filedialog.askopenfilename(initialdir = "/",title = "Select your playlist",filetypes = [("text files","*.txt")])
        with open(filename, "r") as file :
            data = file.read().split("\n")
            tkList.insert("end", *data)

    def cmd_set(self, settings : list) :
        if len(settings) != 2 : raise CustomError("Missing argument for --set command.\nUse this command like : --set ip port")
        self.responder.reset_address(settings[0], settings[1])
    
    def set(self, _ip : str, _port : str, window : tk.Tk):
        self.responder.reset_address(_ip, _port)
        window.destroy()
    
    #Auto multiply the value with 30.
    def cmd_scroll(self,value :int):
        self.responder.post(SCROLL, {"seek": value*30})
    
    def scroll_video(self, command:str, btn:ttk.Button = None) :
        def fff() : self.responder.post(SCROLL, {"seek":  5*30})
        def bbb() : self.responder.post(SCROLL, {"seek": -5*30})
        def bb()  : self.responder.post(SCROLL, {"seek": -1*30})
        def ff()  : self.responder.post(SCROLL, {"seek":  1*30})
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
    def __init__(self, _ip = "http://127.0.0.1:", _port = "5000/"):

        self.ip      =  "http://127.0.0.1:"
        self.port    =  "5000/"
        self.address =  _ip + _port

    def reset_address(self, _ip : str, _port : str) :
        self.ip, self.port, self.address = _ip, _port, _ip + _port

    def post(self, route, msg=None) -> int:
        try:
            r = requests.post(self.address+route, params=msg)
            return r.status_code
        except:
            return 500
        

class CustomError(Exception):
      pass



    
