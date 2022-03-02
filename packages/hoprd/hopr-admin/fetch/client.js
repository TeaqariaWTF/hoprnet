const API_ENDPOINT_PORT = 13301
const API_ENDPOINT = `http://localhost:${API_ENDPOINT_PORT}/api/v2/`
const API_SECURITY_TOKEN = "^^LOCAL-testing-123^^"

export const getReq = (apiSuffix) => {
  return fetch(API_ENDPOINT + apiSuffix, {
    headers: {
      'Accept': 'application/json',
      'x-auth-token': API_SECURITY_TOKEN,
    },
  }).then(res => res.json())
    .then(res => console.log(res));
}

export const postReq = (apiSuffix, jsonBody) => {
  return fetch(API_ENDPOINT + apiSuffix, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-auth-token': API_SECURITY_TOKEN,
    },
    body: JSON.stringify(jsonBody)
  }).then(res => res.json())
    .then(res => console.log(res));
}

export const delReq = (apiSuffix) => {
  return fetch(API_ENDPOINT + apiSuffix, {
    headers: {
      'Accept': 'application/json',
      'x-auth-token': API_SECURITY_TOKEN,
    },
  }).then(res => res.json())
    .then(res => console.log(res));
}

export const parseCmd = (cmdInput) => {
  const split = cmdInput.trim().split(/\s+/)
  const command = split[0]
  const query = split.slice(1).join(' ')

  if (command == null) {
    return undefined
  }

  return {cmd: command, query: query}
 }
