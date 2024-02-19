import { ApiProviderWrapper } from 'wookashwackomytest-polkahat-chai-matchers';

export const getLocalApiProviderWrapper = (port: number) => new ApiProviderWrapper(`ws://127.0.0.1:${port}`);
