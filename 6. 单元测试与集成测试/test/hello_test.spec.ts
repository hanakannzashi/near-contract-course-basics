import { initContext, Context } from './context';

let context: Context;

beforeAll(async () => {
  // 初始化全局上下文
  context = await initContext();
});

afterAll(async () => {
  const { worker } = context;
  // 记得关 sandbox, 不然多跑几次内存就满了
  await worker.tearDown();
});

test('Test setter without permission', async () => {
  const { contract, root } = context;

  const promise = root.call<void>(contract, 'set_account_description', {
    account_id: 'bob.near',
    description: 'Nice Bob',
  });

  await expect(promise).rejects.toThrow('Only contract owner can call this method.');
});

test('Test setter getter', async () => {
  const { contract, alice } = context;

  await alice.call(contract, 'set_account_description', {
    account_id: 'bob.near',
    description: 'Nice Bob',
  });

  const description = await contract.view<string>('get_account_description', {
    account_id: 'bob.near',
  });

  expect(description).toEqual('Nice Bob');
});
