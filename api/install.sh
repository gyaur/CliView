echo "Installing dependencies"

sudo apt update
sudo apt install -y omxplayer libsqlite3-dev

echo "Getting latest version"

mkdir CliView
cd CliView

wget $(curl -s https://api.github.com/repos/gyaur/CliView/releases/latest | grep 'browser_' | cut -d\" -f4)

echo "Starting CLiView"

#TODO: Add systemd support
.\proxy& > \dev\null
.\queue& > \dev\null
.\command& > \dev\null
.\streamer& > \dev\null


echo "All done"