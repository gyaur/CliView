import functionality  # Both the GUI and CLI uses this for communaction

# All test here should return with code 200


def test_pozitive():
    handler = functionality.Functionality()

    # /skip [POST]
    def skip():
        assert(handler.next() == 200)

    # /seek [POST]
    def seek():
        # For the command line
        assert handler.cmd_scroll(1) == 200  # FF
        assert handler.cmd_scroll(5) == 200  # FFF
        assert handler.cmd_scroll(-1) == 200  # BB
        assert handler.cmd_scroll(-5) == 200  # BBB

        # For the GUI
        assert handler.scroll_video("ff") == 200  # FF
        assert handler.scroll_video("fff") == 200  # FFF
        assert handler.scroll_video("bb") == 200  # BB
        assert handler.scroll_video("bbb") == 200  # BBB

    # play / pause [POST]
    def start():
        assert handler.cmd_start() == 200  # Start the music
        assert handler.cmd_stop() == 200  # Stop the music
        # Uses cmd_start() for the GUI
        assert handler.scroll_video("start") == 200
        assert handler.is_music_playing() == False  # Always false

    # /volume [GET]
    def volume():
        assert handler.cmd_volume([0]) == 200
        assert handler.cmd_volume([1]) == 200
        assert handler.cmd_volume([2]) == 200
        assert handler.cmd_volume([3]) == 200
        assert handler.cmd_volume([4]) == 200
        assert handler.cmd_volume([5]) == 200
        assert handler.cmd_volume([6]) == 200
        assert handler.cmd_volume([7]) == 200
        assert handler.cmd_volume([8]) == 200
        assert handler.cmd_volume([9]) == 200
        assert handler.cmd_volume([10]) == 200
        assert handler.cmd_volume([11]) == 400

    skip()
    seek()
    start()
    volume()


if __name__ == "__main__":
    test_pozitive()
    print("Everything passed")
