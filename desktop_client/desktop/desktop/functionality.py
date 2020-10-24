import tkinter as tk
from tkinter import messagebox, ttk, filedialog

class Functionality:
    def __init__() :
        pass

    def error(msg:str) :
        messagebox.showerror(title="Error", message=msg)
    
    def upload(tkList:tk.Listbox, link:str):
        tkList.insert("end", link)

    def cast(tkList:tk.Listbox) :
        link = tkList.get(0)
        tkList.delete(0)
        print(f"{link} is being cast.")

    def previous():
        print("The previous video")

    def next():
        print("The next video")

    def save_links(tkList:tk.Listbox):
        with filedialog.asksaveasfile(mode='w', defaultextension=".txt") as file:
            data  = tkList.get(0, "end")
            file.write("\n".join(data))
    
    def scroll_video(command:str, btn:ttk.Button = None) :
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


       
