#!/usr/bin/env python3
import os.path
import json
import sys
import pprint
import subprocess
import time
import requests

LMS_XAPI_REPO_LOCATION = '../aips-partner-portal-xapi'

def fetch_jwt():
    EXPIRY_DURATION_SECONDS = 150
    JWT_FILE_PATH = "/tmp/jwt"
    '''
    This method will return a JWT, either from file cache (if not expired) or obtain a new one
    '''

    if(os.path.isfile(JWT_FILE_PATH)):
        with open(JWT_FILE_PATH,"r") as f:
            persisted_token = {}
            try: # Handles situtation where JWT file is corrupted
                persisted_token=json.load(f)
            except:
                os.remove(JWT_FILE_PATH)

            if ('expiring at' in persisted_token) and (persisted_token['expiring at'] > time.time()+2):
                return persisted_token['jwt']
    
    jwt = subprocess.check_output('npm run --silent generate-token -- location-management-system jyuen@seek.com.au \'Johnson Yuen\'',shell=True,cwd=LMS_XAPI_REPO_LOCATION).decode("utf-8")
    jwt = jwt[:-1]
    with open(JWT_FILE_PATH,"w") as f:
        persisted_token = {'expiring at':int(time.time()+EXPIRY_DURATION_SECONDS),'jwt':jwt}
        json.dump(persisted_token,f)
    return jwt

try:
    url = 'http://location-management-system.sd.dev.outfra.xyz' + sys.argv[1]
    print(f"Request:\n{url}\nResponse:")
    headers = {'Authorization': 'Bearer '+fetch_jwt()}
    r = requests.get(url, headers=headers)
    str_response = r.text

    parsed = json.loads(str_response)

    pprint.pprint(parsed)
except Exception as e:
    print(str_response)
    print('Usage: lms_curl path. E.g. lms_curl /locations. Make sure you have awsauth -r ADFS-AIPS-Prod-Admin')