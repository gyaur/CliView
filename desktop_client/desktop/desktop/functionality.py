import json
import requests
import tkinter as tk
from tkinter import messagebox, ttk, filedialog
from constants import MCAST, NEXT, SCROLL, VOLUME, P_STATUS, START, STOP, SETTINGS


# This is the backend...
class Functionality:
    def __init__(self):
        self.links = []
        self.responder = Responder()
        self.commands = {"--cast": self.cmd_cast,
                         "--prev": self.previous,
                         "--next": self.next,
                         "--mcast": self.cmd_mcast,
                         "--scroll": self.cmd_scroll,
                         "--set": self.cmd_set,
                         "--volume": self.cmd_volume,
                         "--start": self.cmd_start,
                         "--stop": self.cmd_stop
                         }

    def error(self, msg: str):
        messagebox.showerror(title="Error", message=msg)

    def upload(self, tkList: tk.Listbox, link: str):
        tkList.insert("end", link)

    # Add all links to the queue
    def cast(self, tkList: tk.Listbox):
        self.cmd_mcast(tkList.get(0, "end"))

    # It will cast the video at once
    def cmd_cast(self, link: list):
        err = self.responder.post(MCAST, {"url": link[0]})
        if err != 200:
            raise CustomError(f"An error occurred, server response : {err}")

    # This will add all links to a queue
    def cmd_mcast(self, links: list):
        codes = [self.responder.post(MCAST, {"url": link}) for link in links]
        res = next(((i, x) for (i, x) in enumerate(codes) if x != 200), None)
        if res is not None:
            raise CustomError(
                f"The {res[0]}. link could not be sent to the server. Error code : {res[1]}")

    def cmd_volume(self, value: list):
        err = self.responder.post(VOLUME, {"volume": int(value[0])})
        if err != 200:
            raise CustomError(f"An error occurred, server response : {err}")

    def set_volume(self, volume: str):
        v = int(float(volume))
        self.cmd_volume([v])

    def previous(self):
        pass

    def next(self):
        err = self.responder.post(NEXT)
        if err != 200:
            raise CustomError(f"An error occurred, server response : {err}")

    # Return true if music playing false otherwise
    def is_music_playing(self) -> bool:
        msg, err = self.responder.get(P_STATUS)
        if (err != 200):
            raise CustomError(f"An error occurred, server response : {err}")

        return msg["status"]

    def cmd_start(self):
        if not self.is_music_playing():
            err = self.responder.post(START)
            if err != 200:
                raise CustomError(
                    f"An error occurred, server response : {err}")

    def cmd_stop(self):
        if self.is_music_playing():
            err = self.responder.post(STOP)
            if err != 200:
                raise CustomError(
                    f"An error occurred, server response : {err}")

    def save_links(self, tkList: tk.Listbox):
        with filedialog.asksaveasfile(mode='w', defaultextension=("text files", "*.txt")) as file:
            data = tkList.get(0, "end")
            file.write("\n".join(data))

    def load_links(self, tkList: tk.Listbox):
        tkList.delete(0, "end")

        filename = filedialog.askopenfilename(
            initialdir="/",
            title="Select your playlist",
            filetypes=[
                ("text files",
                 "*.txt")])
        with open(filename, "r") as file:
            data = file.read().split("\n")
            tkList.insert("end", *data)

    def cmd_set(self, settings: list):
        if len(settings) != 2:
            raise CustomError(
                "Missing argument for --set command.\nUse this command like : --set ip port")

        with open(SETTINGS, "r+") as file:
            data = json.load(file)
            data["networking"]["ip"] = settings[0]
            data["networking"]["port"] = settings[1]
            file.seek(0)
            file.truncate()
            json.dump(data, file)

    def set(self, _ip: str, _port: str, window: tk.Tk):
        self.responder.reset_address(_ip, _port)
        self.cmd_set([_ip, _port])
        window.destroy()

    # Auto multiply the value with 30.
    def cmd_scroll(self, value: int):
        self.responder.post(SCROLL, {"ammount": value * 30})

    def scroll_video(self, command: str, btn: ttk.Button = None):
        def fff(): self.cmd_scroll(5)
        def bbb(): self.cmd_scroll(-5)
        def bb(): self.cmd_scroll(-1)
        def ff(): self.cmd_scroll(5)

        def start(btn: ttk.Button):
            cmd, label = "", ""
            if self.is_music_playing():
                cmd = STOP
                label = "START"
            else:
                cmd = START
                label = "STOP"

            btn["text"] = label
            self.responder.post(cmd)

        commands = {"bbb": bbb, "bb": bb, "ff": ff, "fff": fff, "start": start}
        if btn:
            f = commands[command]
            f(btn)
            return
        commands[command]()

# This is the communication between the server and the application


class Responder:
    def __init__(self):
        with open(SETTINGS) as file:
            settings = json.load(file)
            self.ip = settings["networking"]["ip"]
            self.port = settings["networking"]["port"]

        self.address = self.ip + ":" + self.port + "/"

    def reset_address(self, _ip: str, _port: str):
        self.ip, self.port, self.address = _ip, _port, _ip + ":" + _port + "/"

    def post(self, route, msg=None) -> int:
        r = requests.post(self.address + route, json=msg)
        return r.status_code

        '''
        For later...
        try:
            r = requests.post(self.address+route, params=msg)
            return r.status_code
        except:
            return 500
        '''

    def get(self, route, msg=None) -> (dict, int):
        r = requests.get(self.address + route, json=msg)
        return (r.json(), r.status_code)

        '''
        For later...
        try:
            r = requests.get(self.address+route, params=msg)
            return (r.json(), r.status_code)
        except:
            return (None, 500)
        '''


class CustomError(Exception):
    pass
