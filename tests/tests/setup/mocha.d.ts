declare namespace Mocha {
  type InjectableContext = {
    currentFailureCount: number;
  };
  type TestContext = Mocha.Context & InjectableContext;
  /**
   * Callback function used for tests and hooks.
   */
  type Func = (this: TestContext, done: Done) => void;

  /**
   * Async callback function used for tests and hooks.
   */
  type AsyncFunc = (this: TestContext) => PromiseLike<any>;
  interface RootHookObject {
    /**
     * In serial mode, run after all tests end, once only.
     * In parallel mode, run after all tests end, for each file.
     */
    afterAll?: Func | AsyncFunc | Func[] | AsyncFunc[] | undefined;
    /**
     * In serial mode (Mocha's default), before all tests begin, once only.
     * In parallel mode, run before all tests begin, for each file.
     */
    beforeAll?: Func | AsyncFunc | Func[] | AsyncFunc[] | undefined;
    /**
     * In both modes, run after every test.
     */
    afterEach?: Func | AsyncFunc | Func[] | AsyncFunc[] | undefined;
    /**
     * In both modes, run before each test.
     */
    beforeEach?: Func | AsyncFunc | Func[] | AsyncFunc[] | undefined;
  }
}
