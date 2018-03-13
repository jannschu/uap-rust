"""
Downloads the resources from ua-parser.
"""
import json
import yaml
import os
from urllib.request import urlopen
from urllib.parse import urljoin
import re

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
    with open(path, 'w') as file:
        file.write(content)

def yaml_to_json(yaml_str):
    """Convert a YAML to JSON."""
    obj = yaml.load(yaml_str)
    return json.dumps(obj)

def copy(path, patch=None):
    """Copy a file from the remote repo to the local."""
    print("Copy {}...".format(path))
    content = download(path)
    if callable(patch):
        content = patch(content)
    if path.endswith('.yaml'):
        content = yaml_to_json(content)
        path = path[:-5] + '.json'
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

def _patch_regex_file(yaml_str):
    # Fix. See https://github.com/ua-parser/uap-core/pull/310
    yaml_str = yaml_str.replace('|)', ')?')
    yaml_str = re.sub(r"(?<!\\)\\([ /!])", '\\1', yaml_str)
    return yaml_str

if __name__ == '__main__':
    copy("LICENSE")
    copy("regexes.yaml", patch=_patch_regex_file)
    for browser_test_file in BROWSER_TEST_FILES + OS_TEST_FILES + DEVICE_TEST_FILES:
        copy(browser_test_file)
