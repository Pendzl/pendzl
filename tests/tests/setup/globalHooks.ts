import chai from 'chai';
const MAX_FAILURES = process.env.MAX_FAILURES ? parseInt(process.env.MAX_FAILURES) : null;
chai.config.includeStack = true;
export const mochaHooks: Mocha.RootHookObject = {
  beforeAll(this: Mocha.Context) {
    const context: Mocha.InjectableContext = {
      currentFailureCount: 0,
    };

    Object.assign(this, context);
  },
  beforeEach(this: Mocha.TestContext) {
    if (MAX_FAILURES && this.currentFailureCount >= MAX_FAILURES) {
      this.skip();
    }
  },
  afterAll(this: Mocha.TestContext) {
    // the contents of the After All hook
  },
  afterEach(this: Mocha.TestContext) {
    if (this.currentTest?.isFailed()) {
      // console.log('here');
    }

    if (this.currentTest?.isFailed() || !this.currentTest?.isPassed()) {
      this.currentFailureCount++;
    }
  },
};
