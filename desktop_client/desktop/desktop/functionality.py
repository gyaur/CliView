import json
from typing import Dict, Tuple
import requests
from constants import MCAST, NEXT, SCROLL, VOLUME, P_STATUS, START, STOP, SETTINGS
import validators
import re
import socket



# This is the backend...
class Functionality:
    def __init__(self):
        self.links = []
        self.responder = Responder()
        self.commands = {
            "--cast": self.cmd_cast,
            "--prev": self.previous,
            "--next": self.next,
            "--mcast": self.cmd_mcast,
            "--scroll": self.cmd_scroll,
            "--set": self.cmd_set,
            "--volume": self.cmd_volume,
            "--start": self.cmd_start,
            "--stop": self.cmd_stop
        }

    def is_valid_link(self, link: str) -> bool:
        return validators.url(link)

    # It will cast the video at once
    def cmd_cast(self, link: list):
        err = 0
        if self.is_valid_link(link[0]):
            err = self.responder.post(MCAST, {"url": link[0]})
        else:
            err = self.responder.post(
                MCAST, {"url": self.responder.address + link[0]})

        if err != 200:
            raise CustomError(f"An error occurred, server response : {err}")

    # This will add all links to a queue
    def cmd_mcast(self, links: list):
        print([{
            "url": link
        } if self.is_valid_link(link) else {
            "url": self.responder.local_address + link
        } for link in links])
        codes = [
            self.responder.post(MCAST, {"url": link})
            if self.is_valid_link(link) else self.responder.post(
                MCAST, {"url": self.responder.local_address + link})
            for link in links
        ]
        res = next(((i, x) for (i, x) in enumerate(codes) if x != 200), None)
        if res is not None:
            raise CustomError(
                f"The {res[0]}. link could not be sent to the server. Error code : {res[1]}"
            )

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

    def cmd_set(self, settings: list):
        if len(settings) != 2:
            raise CustomError(
                "Missing argument for --set command.\nUse this command like : --set ip port"
            )

        with open(SETTINGS, "r+") as file:
            data = json.load(file)
            data["networking"]["ip"] = settings[0]
            data["networking"]["port"] = settings[1]
            file.seek(0)
            file.truncate()
            json.dump(data, file)

    # Auto multiply the value with 30.
    def cmd_scroll(self, value: int):
        self.responder.post(SCROLL, {"ammount": value * 30})


# This is the communication between the server and the application


class Responder:
    def __init__(self):
        with open(SETTINGS) as file:
            settings = json.load(file)
            self.ip = settings["networking"]["ip"]
            self.port = settings["networking"]["port"]

        self.address = self.ip + ":" + self.port + "/"
        self._set_local_address()

    def reset_address(self, _ip: str, _port: str):
        self.ip, self.port, self.address = _ip, _port, _ip + ":" + _port + "/"

    def _set_local_address(self):
        def get_ip():
            s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            try:
                # doesn't even have to be reachable
                s.connect(('8.8.8.8', 1))
                ip = s.getsockname()[0]
            except Exception:
                ip = '127.0.0.1'
            finally:
                s.close()
                print(ip)
            return f"http://{ip}:8088/"

        self.local_address = get_ip()

    def post(self, route, msg=None) -> int:
        r = requests.post(self.address + route, json=msg)
        return r.status_code
        '''
        http_client.HTTPConnection.debuglevel = 1
        logging.basicConfig()
        logging.getLogger().setLevel(logging.DEBUG)
        requests_log = logging.getLogger("requests.packages.urllib3")
        requests_log.setLevel(logging.DEBUG)
        requests_log.propagate = True


        For later...
        try:
            r = requests.post(self.address+route, params=msg)
            return r.status_code
        except:
            return 500
        '''

    def get(self, route, msg=None) -> Tuple[Dict, int]:
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
