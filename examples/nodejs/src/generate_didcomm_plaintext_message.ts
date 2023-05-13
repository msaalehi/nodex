import * as os from 'os'
import * as path from 'path'
import got from 'got'

(async () => {
    const base = `unix:${ path.join(os.homedir(), '.nodex/run/nodex.sock') }`
    const json = await got.post([ base, '/internal/didcomm/plaintext-messages' ].join(':'), {
        enableUnixSockets: true,
        json: {
            destinations: [ 'did:nodex:test:EiBprXreMiba4loyl3psXm0RsECdtlCiQIjM8G9BtdQplA' ],
            message: {
                string: 'value',
                number: 1,
                boolean: true,
                array: [],
                map: {}
            },
        },
    }).json()

    console.log(JSON.stringify(json, null, 4))
})()
