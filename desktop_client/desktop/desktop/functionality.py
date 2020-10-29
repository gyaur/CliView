import tkinter as tk
from tkinter import messagebox, ttk, filedialog

class Functionality:
    def __init__(self) :
        self.links = []
        self.commands = {"--cast"  : self.cmd_cast,
                         "--prev"  : self.previous,
                         "--next"  : self.next,
                         "--mcast" : self.cmd_list
                         }

    def error(self, msg:str) :
        messagebox.showerror(title="Error", message=msg)
    
    def upload(self, tkList:tk.Listbox, link:str):
        tkList.insert("end", link)

    def cast(self, tkList:tk.Listbox) :
        link = tkList.get(0)
        tkList.delete(0)
        print(f"{link} is being cast.")

    def cmd_list(self, links : list):
        self.links = links
        print(self.links)
    
    def cmd_cast(self, link:list) :
        print(f"{link[0]} is being cast.")

    def previous(self):
        print("The previous video")

    def next(self):
        print("The next video")

    def save_links(self, tkList:tk.Listbox):
        with filedialog.asksaveasfile(mode='w', defaultextension=".txt") as file:
            data  = tkList.get(0, "end")
            file.write("\n".join(data))
    
    def scroll_video(self, command:str, btn:ttk.Button = None) :
        def bbb() : print("Turn back 30s")
        def bb()  : print("Turn back 10s")
        def ff()  : print("Forward 10s")
        def fff() : print("Fast forward 30s")
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


       
