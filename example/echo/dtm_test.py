import multiprocessing as mp
import subprocess

# This CONSTANT must be consistent with the setting

MAX_TRACES = 100
PLAYER_ID = 1
PLAYER_IP = "127.0.0.1"
PLAYER_PORT = 24000

ECHO_SERVER_NODES = [
    (10, 8001, 24001),
    (20, 8002, 24002)
]

TARGET_PATH = "/tmp/echo/echo_server"


def run_echo_server(queue, node_id, echo_listen_port, tested_port, player_id, player_ip, player_port):
    #  echo_server <port> <id> <dtm tested port> <player id> <player ip> <player port>
    cmd = "{} {} {} {} {} {} {}".format(
        TARGET_PATH, echo_listen_port, node_id, tested_port, player_id, player_ip, player_port)
    p = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE)
    result = (str(p.stdout.read()))
    queue.put(result)


def run_echo_servers():
    vec = []
    ctx = mp.get_context('spawn')
    q = ctx.Queue()
    for (node_id, echo_listen_port, tested_port) in ECHO_SERVER_NODES:
        argv = (q, node_id, echo_listen_port, tested_port, PLAYER_ID, PLAYER_IP, PLAYER_PORT)
        p = ctx.Process(target=run_echo_server, args=argv)
        p.start()
        vec.append((node_id, p))

    for (node_id, p) in vec:
        print("node {} output:".format(node_id))
        print(q.get())
        print("")
        p.join()


if __name__ == '__main__':
    for i in range(MAX_TRACES):
        run_echo_servers()
