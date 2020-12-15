import tkinter as tk
from tkinter import messagebox, ttk, filedialog
import functionality as func
from tkinter import ttk
from PIL import Image, ImageTk
import os
from constants import LOW_V, HIGH_V, START_V, START, STOP

class ResposeWriterGUI:
    def __init__(self):
        # Handler
        self.handler = func.Functionality()
        self._job    = None

        # ------------------------------Setting up the main window-------------
        main_window = tk.Tk()
        main_window.title("CliView")
        main_window.geometry("500x700")
        main_window.resizable(False, False)
        #main_window.iconbitmap("Pictures/icon.ico") It only works for Windows.
        main_window.tk.call('wm', 'iconphoto', main_window._w, ImageTk.PhotoImage(file='Pictures/icon.ico'))
        self.app = main_window
        

        #Background image
        foot_img = ImageTk.PhotoImage(Image.open("Pictures/background.png").resize((500, 700), Image.ANTIALIAS))
        canvas = tk.Canvas(main_window, width=500, height=700)
        canvas.create_image(0,0,anchor='nw',image=foot_img)
        canvas.image = foot_img
        canvas.place(x=0, y=0, relwidth=1, relheight=1)
        
        # Setting up frames
        logo_frame = tk.Frame(main_window, width=500, height=200)
        logo_frame.pack(side="top", pady=(5, 25))

        input_frame = tk.Frame(main_window, width=200, height=50)
        input_frame.pack(anchor="center")

        links_frame = tk.Frame(main_window, width=500, height=200)
        links_frame.pack(side="top", anchor="center")

        control_frame = tk.Frame(main_window, width=200, height=50)
        control_frame.pack(anchor="center", pady=(70, 10))

        sound_frame = tk.Frame(main_window, width=200, height=50, bg="white")
        sound_frame.pack(anchor="center")

        master_frame = tk.Frame(main_window, width=200, height=50)
        master_frame.pack(anchor="center", pady=(50, 0))

        # Logo frame
        logo = ImageTk.PhotoImage(Image.open(
            "Pictures/logo.png").resize((150, 150), Image.ANTIALIAS))
        logo_label = tk.Label(logo_frame, image=logo)
        logo_label.image = logo
        logo_label.pack(anchor="center")

        # Links frame
        self.upload_links = ttk.Entry(input_frame, width=45)
        self.upload_links.grid(row=0, column=0, columnspan=2)

        upload_button = ttk.Button(
            input_frame,
            text="UPLOAD",
            width=13,
            command=lambda: self.upload(self.upload_links.get()) if self.upload_links.get() != "" else self.load_local_file())
        upload_button.grid(row=0, column=3)

        cast_button = ttk.Button(input_frame, text="CAST", width=13, command=lambda: self.cast()
             if self.link_list.size() > 0 else self.error("Provide at least one link!"))
        cast_button.grid(row=0, column=4)

        self.link_list = tk.Listbox(
            links_frame,
            selectmode="extended",
            width=75,
            height=7)
        self.link_list.bind("<KeyRelease>", self.delete_element)
        self.link_list.pack(anchor="center")

        # Control frame
        bbb_icon = ImageTk.PhotoImage(Image.open(
            "Pictures/bbb.png").resize((40, 40), Image.ANTIALIAS))
        bb_icon  = ImageTk.PhotoImage(Image.open(
            "Pictures/bb.png").resize((40, 40), Image.ANTIALIAS))
        ff_icon  = ImageTk.PhotoImage(Image.open(
            "Pictures/ff.png").resize((40, 40), Image.ANTIALIAS))
        fff_icon = ImageTk.PhotoImage(Image.open(
            "Pictures/fff.png").resize((40, 40), Image.ANTIALIAS))
        self.strt_icon= ImageTk.PhotoImage(Image.open(
            "Pictures/play.png").resize((40, 40), Image.ANTIALIAS))
        self.stop_icon= ImageTk.PhotoImage(Image.open(
            "Pictures/stop.png").resize((40, 40), Image.ANTIALIAS))

        bbb_button = ttk.Button(
            control_frame,
            image = bbb_icon,
            command=lambda: self.scroll_video("bbb"))
        bbb_button.image = bbb_icon

        bb_button = ttk.Button(
            control_frame,
            image = bb_icon,
            command=lambda: self.scroll_video("bb"))
        bb_button.image = bb_icon

        self.start_button = ttk.Button(
            control_frame,
            image = self.strt_icon,
            command=lambda: self.scroll_video("start"))
        self.start_button.image = self.strt_icon

        ff_button = ttk.Button(
            control_frame,
            image = ff_icon,
            command=lambda: self.scroll_video("ff"))
        ff_button.image = ff_icon

        fff_button = ttk.Button(
            control_frame,
            image = fff_icon,
            command=lambda: self.scroll_video("fff"))
        fff_button.image = fff_icon

        bbb_button.grid(row=0, column=0)
        bb_button.grid(row=0, column=1)
        self.start_button.grid(row=0, column=2)
        ff_button.grid(row=0, column=3)
        fff_button.grid(row=0, column=4)

        # Sound frame
        style = ttk.Style()
        style.configure('TScale', background = 'white') 

        sound_icon = ImageTk.PhotoImage(Image.open(
            "Pictures/max_sound.png").resize((30, 30), Image.ANTIALIAS))
        sound_label = tk.Label(sound_frame, image=sound_icon, bg="white")
        sound_label.image = sound_icon
        sound_label.grid(row=0, column=0)

        self.mixer = ttk.Scale(
            sound_frame,
            from_=LOW_V,
            to=HIGH_V,
            orient="horizontal",
            style='TScale',
            command=self.updateValue)
        self.mixer.set(START_V)
        self.mixer.grid(row=0, column=1)

        # Master frame
        next_button = ttk.Button(
            master_frame,
            text="Next",
            width=50,
            command=self.handler.next)
        save_button = ttk.Button(master_frame, text="Save", width=50, command=lambda: self.save_links()
            if self.link_list.size() > 0 else self.error("Nothing can be saved!"))
        load_button = ttk.Button(
            master_frame,
            text="Load",
            width=50,
            command=lambda: self.load_links())
        set_button = ttk.Button(
            master_frame,
            text="Settings",
            width=50,
            command=self.setting)
        exit_button = ttk.Button(
            master_frame,
            text="Exit",
            width=50,
            command=main_window.destroy)

        next_button.grid(row=0, column=0, columnspan=2)
        save_button.grid(row=1, column=0, columnspan=2)
        load_button.grid(row=2, column=0, columnspan=2)
        set_button.grid(row=3,  column=0, columnspan=2)
        exit_button.grid(row=4, column=0, columnspan=2)

       

    def start(self):
        self.app.mainloop()  # This is the main event loop place your code before this

    def setting(self):
        # ------------------------------Setting up the settings window---------
        set_window = tk.Tk()
        set_window.title("CliView")
        set_window.geometry("300x150")
        set_window.resizable(False, False)
        set_window.attributes("-topmost", True)

        # Setting up frames
        input_frame = tk.Frame(set_window, width=200, height=50)
        input_frame.pack(anchor="center")

        set_frame = tk.Frame(set_window, width=150, height=50)
        set_frame.pack(anchor="center", pady=(50, 0))

        # Input frame
        ip_label = tk.Label(input_frame, text="Ip address:")
        ip_input = ttk.Entry(input_frame, width=35)
        ip_input.insert(0, self.handler.responder.ip)

        port_label = tk.Label(input_frame, text="Port number:")
        port_input = ttk.Entry(
            input_frame,
            width=35,
            text=self.handler.responder.port)
        port_input.insert(0, self.handler.responder.port)

        ip_label.grid(row=0, column=0)
        ip_input.grid(row=0, column=1)
        port_label.grid(row=1, column=0)
        port_input.grid(row=1, column=1)

        # Set frame
        ok_button = ttk.Button(
            set_frame,
            text="OK",
            width=25,
            command=lambda: self.set(
                ip_input.get(),
                port_input.get(),
                set_window) if ip_input.get() != "" and port_input.get() != "" else self.error("Empty input fields!"))
        ok_button.pack(anchor="center")

        self.setting_window = set_window
        self.setting_window.mainloop()

    def updateValue(self, event):
        if self._job:
            self.app.after_cancel(self._job)
        self._job = self.app.after(500, lambda: self.handler.set_volume(self.mixer.get()))

    def scroll_video(self, command: str):
        def fff(): self.handler.cmd_scroll(5)
        def bbb(): self.handler.cmd_scroll(-5)
        def bb(): self.handler.cmd_scroll(-1)
        def ff(): self.handler.cmd_scroll(5)

        def start():
            cmd = ""

            if self.handler.is_music_playing():
                cmd = STOP
                self.start_button.configure(image = self.strt_icon)
               
            else:
                cmd = START
                self.start_button.configure(image = self.stop_icon)

            self.handler.responder.post(cmd)

        commands = {"bbb": bbb, "bb": bb, "ff": ff, "fff": fff, "start": start}
        commands[command]()

    def upload(self, link: str):
        self.link_list.insert("end", link)

    # Add all links to the queue
    def cast(self):
        self.handler.cmd_mcast(self.link_list.get(0, "end"))

    def load_links(self):
        self.link_list.delete(0, "end")

        filename = filedialog.askopenfilename(
            initialdir="/",
            title="Select your playlist",
            filetypes=[
                ("text files",
                 "*.txt")])
        if filename == "" : return

        with open(filename, "r") as file:
            data = file.read().split("\n")
            self.link_list.insert("end", *data)

    def load_local_file(self):
        filename = filedialog.askopenfilename(
            initialdir=".",
            title="Select your playlist")

        if filename == '' : return
        filename = filename.split("/")[-1]
        self.link_list.insert("end", filename)

    def set(self, _ip: str, _port: str, window: tk.Tk):
        self.handler.responder.reset_address(_ip, _port)
        self.handler.cmd_set([_ip, _port])
        window.destroy()

    def save_links(self):
        file = filedialog.asksaveasfile(mode='w', defaultextension=[("text files", "*.txt")], filetypes=[("text files","*.txt")])
        if not file : return
        data = self.link_list.get(0, "end")
        file.write("\n".join(data))

    def error(self, msg: str):
        messagebox.showerror(title="Error", message=msg)

    def delete_element(self,e):
        if e.keysym == "Delete" or e.keysym == "d":
            items = self.link_list.get(0, "end")
            indxs =  [idx for idx in self.link_list.curselection()]
            keep  = [x for i,x in enumerate(items) if i not in indxs]
            self.link_list.delete(0,"end")
            self.link_list.insert("end", *keep)


            