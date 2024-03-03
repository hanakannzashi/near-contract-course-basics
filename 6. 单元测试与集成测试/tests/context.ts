import fs from 'fs';
import { NearAccount, Worker } from 'near-workspaces';

export interface Context {
  worker: Worker;
  root: NearAccount;
  contract: NearAccount;
  alice: NearAccount;
}

export async function initContext(): Promise<Context> {
  const worker = await Worker.init({
    network: 'sandbox',
    rm: true, // 关闭 sandbox 后删除测试缓存文件, 节约硬盘空间
  });

  // 默认根账户名为 `test.near`
  const root = worker.rootAccount;

  // `hello_test.test.near`
  const contract = await root.createSubAccount('hello_test');

  // `alice.test.near`
  const alice = await root.createSubAccount('alice');

  const code = fs.readFileSync('res/hello_test.wasm');

  // 部署并初始化合约
  const result = await contract
    .batch(contract.accountId)
    // Deploy Action
    .deployContract(code)
    // FunctionCall Action
    .functionCall('init', {
      owner_id: alice.accountId,
    })
    // 执行交易
    .transact();

  if (result.succeeded) {
    console.log('Succeed to init contract');
  } else {
    throw Error(`Failed to init contract: ${result.Failure?.error_message}`);
  }

  return { worker, root, contract, alice };
}
