echo "Installing dependencies"

sudo apt update
sudo apt install -y omxplayer libsqlite3-dev
pip3 install youtube-dl
sudo ln -sf /home/pi/.local/bin/youtube-dl /usr/bin


mkdir CliView
cd CliView

echo "Getting latest version"

wget $(curl -s https://api.github.com/repos/gyaur/CliView/releases/latest | grep 'browser_' | cut -d\" -f4) -q

sudo chmod +x *

echo "Setting up systemd services"

# Proxy unit file
cat << EOM > cliview_proxy.service
[Unit]
Description=CliView proxy service

[Service]
ExecStart=/home/pi/CliView/proxy
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOM

# Queue unit file
cat <<- EOM > cliview_queue.service
    [Unit]
    Description=CliView queue service
    Requires=cliview_proxy.service

    [Service]
    ExecStart=/home/pi/CliView/queue
    Restart=on-failure

    [Install]
    WantedBy=multi-user.target
EOM


# Command unit file
cat <<- EOM > cliview_command.service
    [Unit]
    Description=CliView command service
    Requires=cliview_proxy.service

    [Service]
    ExecStart=/home/pi/CliView/command
    Restart=on-failure

    [Install]
    WantedBy=multi-user.target
EOM

# Streamer unit file
cat <<- EOM > cliview_streamer.service
    [Unit]
    Description=CliView queue
    Requires=cliview_proxy.service
    Requires=cliview_queue.service
    Requires=cliview_command.service

    [Service]
    ExecStart=/home/pi/CliView/streamer
    Restart=on-failure

    [Install]
    WantedBy=multi-user.target
EOM


sudo mv *.service /etc/systemd/system/

# Setup all the services
sudo systemctl daemon-reload

sudo systemctl --now enable cliview_proxy.service
sudo systemctl --now enable cliview_queue.service
sudo systemctl --now enable cliview_command.service
sudo systemctl --now enable cliview_streamer.service

echo "All done"
echo "To start streaming send requests to raspberrypi.local:5000"