"""
Downloads the resources from ua-parser.
"""
import os
import re
import sys
from urllib.request import urlopen
from urllib.parse import urljoin

try:
    import msgpack
    import yaml
except ModuleNotFoundError:
    sys.exit("You need to install the Python packages `msgpack' and `PyYAML` for this tool.\n"
             "Try to install them with `pip install msgpack PyYAML`.")

def download(path):
    """Download file relative to ua-parser GitHub repository."""
    repo_url = "https://github.com/ua-parser/uap-core/raw/master/"
    url = urljoin(repo_url, path)
    return urlopen(url).read().decode('utf-8')

def save(path, content):
    """Save content at given path relative to /resources."""
    directory = os.path.join(os.path.dirname(__file__), 'resources')
    path = os.path.join(directory, path)
    directory = os.path.dirname(path)
    if not os.path.exists(directory):
        os.makedirs(directory)
    flags = 'wb' if isinstance(content, bytes) else 'w'
    with open(path, flags) as file:
        file.write(content)

def yaml_to_msgpack(yaml_str, patch=None):
    """Convert a YAML to MessagePack format."""
    print("Parse YAML. ", end='', flush=True)
    obj = yaml.load(yaml_str)
    if callable(patch):
        print("Patch. ", end='', flush=True)
        obj = patch(obj)
    print("Convert to MsgPack. ", end='', flush=True)
    return msgpack.packb(obj)

def copy(path, patch=None):
    """Copy a file from the remote repo to the local."""
    print("Copy {}".format(path))
    print("  Download. ", end='', flush=True)
    content = download(path)
    if path.endswith('.yaml'):
        content = yaml_to_msgpack(content, patch=patch)
        path = path[:-5] + '.msgpack'
    print("Save.", flush=True)
    save(path, content)

BROWSER_TEST_FILES = [
    'test_resources/firefox_user_agent_strings.yaml',
    'tests/test_ua.yaml',
    'test_resources/pgts_browser_list.yaml',
    'test_resources/opera_mini_user_agent_strings.yaml',
    'test_resources/podcasting_user_agent_strings.yaml'
]

OS_TEST_FILES = [
    'tests/test_os.yaml', 'test_resources/additional_os_tests.yaml'
]

DEVICE_TEST_FILES = ['tests/test_device.yaml']

KEY_SHORTCUTS = {
    "user_agent_parsers": "b",
    "device_parsers": "d",
    "os_parsers": "o",
    "regex": "r",
    "family_replacement": "f",
    "v1_replacement": "1",
    "v2_replacement": "2",
    "v3_replacement": "3",
    "os_replacement": "o",
    "os_v1_replacement": "1",
    "os_v2_replacement": "2",
    "os_v3_replacement": "3",
    "os_v4_replacement": "4",
    "device_replacement": "d",
    "brand_replacement": "b",
    "model_replacement": "m",
}

def _patch_regex_file(obj):
    if isinstance(obj, dict):
        if 'regex' in obj:
            # Fix. See https://github.com/ua-parser/uap-core/pull/310
            regex = obj['regex'].replace('|)', ')?')
            obj['regex'] = re.sub(r"(?<!\\)\\([ /!])", '\\1', regex)
        if 'regex_flag' in obj:
            # Use the syntax for flags used by Rust's regex implementation
            obj['regex'] = "(?{}){}".format(obj['regex_flag'], obj['regex'])
            del obj['regex_flag']
        for key in list(obj.keys()):
            obj[key] = _patch_regex_file(obj[key])
            # Replace key names by shortcuts to save storage
            if key in KEY_SHORTCUTS:
                obj[KEY_SHORTCUTS[key]] = obj[key]
                del obj[key]
        return obj
    elif isinstance(obj, list):
        return [_patch_regex_file(item) for item in obj]
    else:
        return obj

if __name__ == '__main__':
    copy("LICENSE")
    copy("regexes.yaml", patch=_patch_regex_file)
    for browser_test_file in BROWSER_TEST_FILES + OS_TEST_FILES + DEVICE_TEST_FILES:
        copy(browser_test_file)
