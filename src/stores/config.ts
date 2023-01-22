import type { AWSCredentials } from '..//types/data';
import { writable } from 'svelte/store';
import { ConfigAPI } from '../api/ConfigAPI';
import { createAsyncWritable } from '../utils/createAsyncWritable';

const Config = new ConfigAPI();

const currentCreds = writable<AWSCredentials>();
export const creds = createAsyncWritable<AWSCredentials | undefined>(currentCreds, Config.getCredentials, undefined);