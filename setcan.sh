ip link set can0 down
ip link set can0 up type can bitrate 500000 dbitrate 2000000 fd on
ip link set can0 up
ip -details link show can0
echo 4096 > /sys/class/net/can0/tx_queue_len
