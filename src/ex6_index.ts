import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import type { FrameSystemAccountInfo } from "@polkadot/types/lookup";
import '@polkadot/api-augment'

const sleep = (ms: number) => new Promise(r => setTimeout(r,ms));

const WEB_SOCKET = 'ws://127.0.0.1:9944';

const connect = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    return api;
}

const getConst =async (api:ApiPromise) => {
    const existentialDeposit = await api.consts.balances.existentialDeposit.toHuman();
    return existentialDeposit;
}

const getFreeBalance = async (api: ApiPromise, address: string) => {
    const { data: { free, }, }: FrameSystemAccountInfo = await api.query.system.account(address);
    return free;
}

const transfer = async (api: ApiPromise, alice: KeyringPair, bob: string, amount: number) => {
    await api.tx.balances.transfer(bob, amount).signAndSend(alice, res => {
        console.log(`Tx status: ${res.status}`);
    })
}

const getMetadata = async (api:ApiPromise) => {
    const metadata = await api.rpc.state.getMetadata();
    return metadata.toString();
}

const subscribe = async (api:ApiPromise, address:string) => {
    await api.query.system.account(address, aliceInfo => {
        const free = aliceInfo.data.free;
        console.log('free balance is: ', free.toHuman());
    })
}

const subscribeEvent = async (api:ApiPromise) => {
    await api.query.system.events(events => {
        events.forEach(function(event){
            /*
            event  {
                phase: { ApplyExtrinsic: '1' },
                event: {
                    method: 'SomethingStored',
                    section: 'templateModule',
                    index: '0x0800',
                    data: [ '101', '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' ]
                },
                topics: []
            }
             */
            // console.log('event ',event.toHuman());
            const eventSection = event['event']['section'];
            const eventMethod = event['event']['method'];
            if (eventSection == 'templateModule' && eventMethod == 'SomethingStored') {
                const data = event['event']['data'][0];
                console.log(eventSection + '.' + eventMethod + ': value is ' + data.toHuman());
            }
        });
    })
}

const main = async () => {
    const api = await connect();
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    await subscribeEvent(api);
    await sleep(500000);
    
    console.log('main func');
}

main()
    .then(() => {
        console.log('exist with success');
        process.exit(0);
    })
    .catch(err => {
        console.log('error is ', err);
        process.exit(1);
    })