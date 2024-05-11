
import automata_py
import subprocess

this_node_id = 2
player_node_id = 1

AUTO_ECHO = "echo"

automata_py.automata_setup_with_input(AUTO_ECHO, this_node_id, "127.0.0.1:24002", player_node_id, "127.0.0.1:24001")

print("setup ok")

loop = True
while loop:
    (source, dest, action, message) = automata_py.automata_next_input(AUTO_ECHO)

    print((source, dest, action, message))

    automata_py.automata_action_to_player(
        automata_py.action_input(),
        automata_py.action_end(),
        this_node_id,
        source,
        dest,
        message
    )

    assert action == automata_py.action_input()

    # <port> <id> <player id> <player ip> <player port>
    cmd = "echo_server 8001 1 2 127.0.0.1 8002"
    t = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE).stdout.read()
    print(t)
