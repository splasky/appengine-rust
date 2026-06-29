// Copyright 2026 HY Chang
// SPDX-License-Identifier: Apache-2.0

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const backendUrl = env.BACKEND_URL || 'http://localhost:8080';
    const target = new URL(url.pathname + url.search, backendUrl);
    return fetch(target.toString(), {
      method: request.method,
      headers: request.headers,
      body: request.body,
    });
  },
};

interface Env {
  BACKEND_URL: string;
}
