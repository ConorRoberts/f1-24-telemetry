import createClient from "openapi-fetch";
import { config } from "../config";
import type { paths } from "../openapi";

export const api = createClient<paths>({ baseUrl: config.apiHost });
