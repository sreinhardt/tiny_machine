#!env python3

import socket, re, itertools, time, pickle

LOCAL = False
LVL = 3 # 1, 2, 3
VERBOSE = False
if LVL == 1:
    HOST = "localhost" if LOCAL else "300bcc4101e7955589bde7bdcd9cafe6.bsidescharm.40bytectf.com"
    PORT = 12346 # lvl1
elif LVL == 2:
    HOST = "localhost" if LOCAL else ".bsides.40bytectf.com"
    PORT = 45678 #lvl2
elif LVL == 3:
    HOST = "localhost" if LOCAL else "76324f7033e3f6c5d01fd3983ba6689e.bsidescharm.40bytectf.com"
    PORT = 61830 #lvl3
else:
    HOST = "localhost"
    PORT = 8000 # default

MSG_SIZE = 10
REG_SIZE = 2
MEM_SIZE = 8

def verbose(s):
    if VERBOSE:
        print(s)

class Fuzzer():
    def __init__(self, buf=None, idx=0, limit=None):
        self.limit = limit
        self.rounds = 0
        self.wins = 0
        self.loses = 0
        self.invalid = 0
        self.saves = []

    def generate(self):
        for seq in itertools.product("01", repeat=MSG_SIZE*8):
            bits = ''.join(reversed(seq))
            bits = [bits[i:i+8] for i in range(0, len(bits), 8)]
            bits = [int(b, 2) for b in bits]
            bits = bytes(bits)
            #verbose("mach: {}".format(bits))
            yield bits

    def reg_generate(self):
        for seq in itertools.product("01", repeat=REG_SIZE*8):
            bits = ''.join(reversed(seq))
            bits = [bits[i:i+8] for i in range(0, len(bits), 8)]
            bits = [int(b, 2) for b in bits]
            bits = bytes(bits)
            #verbose("reg: {}".format(bits))
            yield bits

    def mem_generate(self):
        for seq in itertools.product("01", repeat=MEM_SIZE*8):
            bits = ''.join(reversed(seq))
            bits = [bits[i:i+8] for i in range(0, len(bits), 8)]
            bits = [int(b, 2) for b in bits]
            bits = bytes(bits)
            #verbose("mem: {}".format(bits))
            yield bits

    def play(self, buf):
        if len(buf) < MSG_SIZE:
            raise ValueError(buf)
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((HOST,PORT))
        sock.recv(50) # submit tinymachine here
        sock.send(buf)
        resp = sock.recv(200).decode('utf-8') # game response
        if "40ByteCTF" in resp:
            self.wins += 1
            #self.saves.append(buf)
        elif "Invalid instruction" in resp:
            self.invalid += 1
        else:
            self.loses += 1
        self.rounds += 1

    def _fuzz_(self, buf):
        verbose(buf)
        self.play(buf)

    def reg_fuzz(self, mem):
        while not self.limit or self.rounds <= self.limit:
            for reg in self.reg_generate():
                self._fuzz_(reg+mem)

    def mem_fuzz(self, reg):
        while not self.limit or self.rounds <= self.limit:
            for mem in self.mem_generate():
                self._fuzz_(reg+mem)


    def fuzz(self, reg=None, mem=None):
        start = time.time()
        print("Starting fuzzer: {}".format(start))
        try:
            if reg:
                self.mem_fuzz(reg)
            elif mem:
                self.reg_fuzz(mem)
            else:
                while not self.limit or self.rounds <= self.limit:
                    for buf in self.generate():
                        verbose(buf)
                        self.play(buf)
        except ValueError as e:
            print("buff to small dummy: {}".format(e))
        except KeyboardInterrupt:
            pass
        #pickle.dump(self.saves, open('saves.pickle', 'wb'))
        end = time.time()
        print("Ending fuzzer: {}\nRan for: {}".format(end, end-start))

if __name__ == "__main__":
    fuzzer = Fuzzer()
    #fuzzer.fuzz(reg=b'\x00\x00')
    #fuzzer.fuzz(mem=b'\x55\x00\x00\x00\x00\x00\x00\x00')
    #fuzzer.fuzz()

    if LVL == 1:
        # Level 1 fuzzer \x??40ByteCTF
        fuzzer.fuzz(mem=b'0ByteCTF')
    elif LVL == 2:
        # Level 2 fuzzer \x0bsi\xdes\xc0h\xa0rm
        fuzzer.fuzz(mem=b'i\xdes\xc0h\xa0rm')
    elif LVL == 3:
        # Level 3 fuzzer, mem is most minimal asm
        fuzzer.fuzz(mem=b'\x62\x67\x30\xEF\xD3\xFF\x00\x00')
        #fuzzer.play(b'\x00\x00\x62\x67\x30\xEF\xD3\xFF\x00\x00')
    else:
        fuzzer.fuzz()

    print("Fuzzing rounds: {}  Wins: {}  Loses: {}  Invalid: {}".format(
        fuzzer.rounds, fuzzer.wins, fuzzer.loses, fuzzer.invalid))

# |54321|00000| -> |00000|12345|
# |00|00|--|62|67|30|EF|D3|8F|00|00|
#          |01|23|45|67|89|ab|cd|ef|
# get ac # 6  # grab from inb to ac        # 6 calls
# jze 6  # 26 # if 0 jump past jnz         # 6 calls
# put ac # 7  # put ac non-zero into out   # 5 calls
# jnz 0  # 30 # loop for all of inb        # 5 calls
# ldl B  # EF # load li = [B]              # 1 call
# del    # D  # li = li - 1                # 15 calls
# jze 8  # 38 # while li != 0              # 15 calls
# fla    # F  # flip ac = F                # 1 call
# hlt    # 0  # halt                       # 1 call
# hlt    # 0  # halt                       # 0 calls
# hlt    # 0  # halt                       # 0 calls
# hlt    # 0  # halt                       # 0 calls