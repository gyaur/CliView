import sys
import gui
import functionality

try:
    if len(sys.argv) > 1 :
        handler = functionality.Functionality()
        if len(sys.argv[2:]) == 0:
            handler.commands[sys.argv[1]]()
        else:
            handler.commands[sys.argv[1]](sys.argv[2:])
           
    else:
        _gui = gui.ResposeWriterGUI()
        _gui.start()

except Exception as e:
   print(e)
    


