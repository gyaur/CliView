import tkinter       as tk
import functionality as func
from tkinter import ttk
from PIL     import Image, ImageTk
import os
from constants import LOW_V, HIGH_V, START_V

class ResposeWriterGUI :
    def __init__(self):
        #Handler
        self.handler = func.Functionality()

        #------------------------------Setting up the main window------------------------------
        main_window = tk.Tk()
        main_window.title("CliView")
        main_window.geometry("500x700")
        main_window.resizable(False, False)
        #main_window.config(bg="skyblue")
        
        #Styles :
        mixer_style = ttk.Style().configure('Horizontal.TScale',
                                           sliderthickness=100)
        
        #Setting up frames
        logo_frame = tk.Frame(main_window, width=500, height=200, bg="grey")
        logo_frame.pack(side="top", fill="x")
        
        input_frame = tk.Frame(main_window, width=200, height=50)
        input_frame.pack(anchor="center")
        
        links_frame = tk.Frame(main_window, width=500, height=200)
        links_frame.pack(side="top", fill="x", anchor="center")
        
        control_frame = tk.Frame(main_window, width=200, height=50)
        control_frame.pack(anchor="center", pady=(50,0))
        
        sound_frame = tk.Frame(main_window, width=200, height=50)
        sound_frame.pack(anchor="center")
        
        master_frame = tk.Frame(main_window, width=200, height=50)
        master_frame.pack(anchor="center", pady=(50,0))
        
        
        #Logo frame
        logo       = ImageTk.PhotoImage(Image.open("Pictures/logo.png").resize((200, 200), Image.ANTIALIAS))
        logo_label = tk.Label(logo_frame,  bg="#87CEFA", image=logo)
        logo_label.image = logo
        logo_label.pack(anchor="center")
        
        
        #Links frame
        upload_links = ttk.Entry(input_frame, width=45)
        upload_links.grid(row=0, column=0, columnspan=2)
        
        upload_button = ttk.Button(input_frame, text="UPLOAD", width=13, command=lambda :  self.handler.upload(link_list, upload_links.get()) if upload_links.get() != "" else self.handler.error("Provide a link!"))
        upload_button.grid(row=0, column=3)
        
        cast_button = ttk.Button(input_frame, text="CAST", width=13, command=lambda :self.handler.cast(link_list) if link_list.size() > 0 else self.handler.error("Provide at least one link!"))
        cast_button.grid(row=0, column=4)
        
        link_list    = tk.Listbox(links_frame, selectmode="browse", width=75, height=7)
        link_list.pack(anchor="center")
        
        #Control frame
        bbb_button   = ttk.Button(control_frame, text="BBB",   command=lambda : self.handler.scroll_video("bbb"))
        bb_button    = ttk.Button(control_frame, text="BB",    command=lambda : self.handler.scroll_video("bb"))
        start_button = ttk.Button(control_frame, text="START", command=lambda : self.handler.scroll_video("start", start_button))
        ff_button    = ttk.Button(control_frame, text="FF",    command=lambda : self.handler.scroll_video("ff"))
        fff_button   = ttk.Button(control_frame, text="FFF",   command=lambda : self.handler.scroll_video("fff"))
        
        bbb_button.grid(row=0, column=0)
        bb_button.grid(row=0, column=1)
        start_button.grid(row=0, column=2)
        ff_button.grid(row=0, column=3)
        fff_button.grid(row=0, column=4)
        
        #Sound frame
        sound_icon  = ImageTk.PhotoImage(Image.open("Pictures/max_sound.png").resize((30, 30), Image.ANTIALIAS))
        sound_label = ttk.Label(sound_frame, image=sound_icon)
        sound_label.image = sound_icon
        sound_label.grid(row=0, column=0)
        
        mixer = ttk.Scale(sound_frame, from_=LOW_V, to=HIGH_V, orient="horizontal", style=mixer_style, command=lambda value_str:self.handler.set_volume(value_str))
        mixer.set(START_V)
        mixer.grid(row=0, column=1)

        #Master frame
        prev_button = ttk.Button(master_frame, text="Previous",width = 24,  command=self.handler.previous)
        next_button = ttk.Button(master_frame, text="Next",    width = 24,  command=self.handler.next)
        save_button = ttk.Button(master_frame, text="Save",    width = 50,  command=lambda : self.handler.save_links(link_list) if link_list.size() > 0 else self.handler.error("Nothing can be saved!"))
        load_button = ttk.Button(master_frame, text="Load",    width = 50,  command=lambda : self.handler.load_links(link_list))
        set_button  = ttk.Button(master_frame, text="Settings",width = 50,  command=self.setting)
        exit_button = ttk.Button(master_frame, text="Exit",    width = 50,  command=main_window.destroy)
        
        prev_button.grid(row=0, column=0)
        next_button.grid(row=0, column=1)
        save_button.grid(row=1, column=0, columnspan=2)
        load_button.grid(row=2, column=0, columnspan=2)
        set_button.grid( row=3, column=0, columnspan=2)
        exit_button.grid(row=4, column=0, columnspan=2)
        
        self.app = main_window
    
    def start(self):
        self.app.mainloop()     #This is the main event loop place your code before this

    def setting(self):
        #------------------------------Setting up the settings window------------------------------
        set_window = tk.Tk()
        set_window.title("CliView")
        set_window.geometry("300x150")
        set_window.resizable(False, False)
        set_window.attributes("-topmost", True)

        #Setting up frames
        input_frame = tk.Frame(set_window, width=200, height=50)
        input_frame.pack(anchor="center")

        set_frame  =  tk.Frame(set_window, width=150, height=50)
        set_frame.pack(anchor="center", pady=(50,0))

        #Input frame
        ip_label   = tk.Label(input_frame, text="Ip address:")
        ip_input   = ttk.Entry(input_frame, width=35)
        ip_input.insert(0, self.handler.responder.ip)

        port_label = tk.Label(input_frame, text="Port number:")
        port_input = ttk.Entry(input_frame, width=35, text=self.handler.responder.port)
        port_input.insert(0, self.handler.responder.port)

        ip_label.grid(row=0, column=0)
        ip_input.grid(row=0, column=1)
        port_label.grid(row=1, column=0)
        port_input.grid(row=1, column=1)

        #Set frame
        ok_button = ttk.Button(set_frame, text="OK",width = 25,  command=lambda : self.handler.set(ip_input.get(), port_input.get(), set_window)
                                                                                  if ip_input.get() != "" and port_input.get() != ""
                                                                                  else self.handler.error("Empty input fields!"))
        ok_button.pack(anchor="center")

        self.setting_window = set_window
        self.setting_window.mainloop()


        



