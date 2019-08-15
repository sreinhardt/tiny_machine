#!env python3

import r2pipe
import re
from os import listdir
from os.path import isfile, islink, join

BIN_NAME = 'TinyMachine'       # name of rust binary
DEBUG_PATH = "./target/debug/" # path from execution to dbg targets
VERBOSE = True      # print verbose messages
INDENT = 0          # indentation leve for printing
USE_PROJECTS = True # auto open/close projects for use with cli, disable for CI

def verbose(*args, pre_inc=False, pre_dec=False, post_inc=False, post_dec=False):
    if VERBOSE:
        pprint(*args, pre_inc=pre_inc, pre_dec=pre_dec, post_inc=post_inc, post_dec=post_dec)

def pprint(*args, pre_inc=False, pre_dec=False, post_inc=False, post_dec=False):
    global INDENT
    INDENT = INDENT-1 if pre_dec else INDENT
    INDENT = INDENT+1 if pre_inc else INDENT
    # ingest format string and indent appropriately
    #print("args: ", args) # debug
    fmt = "{}{}".format(' '*INDENT, args[0])
    #print("fmt: ", fmt) # debug
    # update args, removing fmt string, () if only arg
    args = tuple(a for i,a in enumerate(args) if i!=0) if 2<=len(args) else tuple()
    #print("args: ", args) # debug
    print(fmt.format(*args))
    INDENT = INDENT-1 if post_dec else INDENT
    INDENT = INDENT+1 if post_inc else INDENT

def ident_files(path=DEBUG_PATH, bin=BIN_NAME):
    files = {
        'library': r'\.rlib$',
        'test-bin': bin+r'\-([\w\d]+(?!\.d|\.dSYM))$',
        'bin': bin+r'$',
    }
    localfiles = [ f for f in listdir(path) if isfile(join(path, f)) or islink(join(path, f)) ]
    # replace files{regex} with files{[path]}
    for key in files.keys():
        regex = re.compile(files[key])
        files[key] = [ f for f in localfiles if regex.search(f) ]
    # filter test-bin for non-dSYM variants
    tmp = list()
    for testfile in files['test-bin']:
        keep = True
        for f in localfiles:
            keep = False if f == testfile+'.dSYM' else keep
        if keep:
            tmp.append(testfile)
    files['test-bin'] = tmp
    verbose("Found files: {}", files)
    return files

def find_test_bin(bins):
    if not isinstance(bins, list):
        raise TypeError
    selection = None
    while not selection:
        print("Possible test binaries:")
        for i,f in enumerate(bins):
            print("\t{}: {}".format(i, f))
        sel = input("Which bin to debug: ");
        if isinstance(sel, str):
            sel = int(sel)
        if 0 <= sel and sel < len(bins):
            selection = sel
    return bins[selection]

def r2open(path, analyze=True, silent=True):
    r2 = r2pipe.open(path, ['-2']) if silent else r2pipe.open(path)
    if analyze:
        r2.cmd('aaaa')
    if USE_PROJECTS:
        r2.cmd('Po {}'.format(path))
    return r2

def r2quit(r2, path, save=True):
    if USE_PROJECTS:
        r2.cmd('Ps {}'.format(path))
    r2.quit()

def ident_fns(r2, name):
    if isinstance(name, bytes):
        name = name.decode('utf-8')
    if not isinstance(name, str):
        name = str(name)
    tmp = list()
    fns = r2.cmd('afl~'+name)
    for fn in fns.split('\n'):
        fn = fn.strip().split(' ')
        tmp.append({
            'address': fn[0],
            'symbol': fn[-1],
        })
    fns = tmp
    verbose("Functions:")
    [verbose("{}", fn) for fn in fns]
    return fns

def set_bp(r2, addr, remove=False):
    if not isinstance(addr, list):
        addr = [addr]
    cmd=""
    for adr in addr:
        if isinstance(adr, int):
            adr = hex(adr)
        cmd += 'db -'+adr+'; ' if remove else 'db '+adr+'; '
    pprint("set_bp: {}", cmd)
    r2.cmd(cmd)

def dbg_test_set_loc(path, funcs):
    dbg_names = ['::test::set_loc::', '::Memory::set_loc::']
    fnames = list()
    r2 = r2open(path, True, True)
    # filter global funcs for dbg_names, keeping matching global names
    for fn in funcs if 'symbol' in fn:
        fn = fn['symbol']
        [fnames.append(fn) for dbg in dbg_names if dbg in fn]
    set_bp(r2, fnames)
    # call sym.__LT_TinyMachine..memory..Memory_u20_as_u20_core..default..Default_GT_::default

    r2quit(r2, path)
    pprint("Success!")

if __name__ == '__main__':
    # parse files for test_bin location
    files = ident_files(DEBUG_PATH, BIN_NAME)
    test_bin = files['test-bin']
    test_bin = test_bin[0] if len(test_bin)==1 else find_test_bin(test_bin)
    test_bin = DEBUG_PATH+test_bin
    # init radare2 and analyze for functions once
    r2 = r2open(test_bin, True, True)
    funcs = ident_fns(r2, BIN_NAME)
    r2.quit()
    # execute debug test cases
    dbg_cases = {
        'dbg_test_set_loc': True,
        'dbg_totally_false': False,
    }
    pprint("### Starting test debugging ###", post_inc=True)
    for dbg in [ key for key in dbg_cases.keys() if dbg_cases[key] ]:
        pprint("#### Started {} debugging ####".format(dbg), post_inc=True)
        locals()[dbg](test_bin, funcs)
        pprint("#### Completed {} debugging ####".format(dbg), pre_dec=True)
    pprint("### Completed test debugging ###", pre_dec=True)