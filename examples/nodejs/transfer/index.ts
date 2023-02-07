import * as os from 'os'
import * as path from 'path'
import got from 'got'

(async () => {
    const base = `unix:${ path.join(os.homedir(), '.unid/run/unid.sock') }`
    const json = await got.post([ base, '/transfer' ].join(':'), {
        enableUnixSockets: true,
        json: {
            destinations: [ 'did:unid:test:EiBprXreMiba4loyl3psXm0RsECdtlCiQIjM8G9BtdQplA' ],
            messages: [ {
                string: 'value',
                number: 1,
                boolean: true,
                array: [],
                map: {}
            }, {
                string: 'value',
                number: 1,
                boolean: true,
                array: [],
                map: {}
            } ],
            metadata: {
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