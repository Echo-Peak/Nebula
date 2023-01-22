
type ApiResponse = {
  name: string,
  data: unknown
}

const POST = async (body: Record<string, unknown>): Promise<ApiResponse> => {
  const response = await fetch('/api', {
    method: 'POST',
    body: JSON.stringify(body),
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    }
  });
  return await response.json();
}

export const fetchAPI = {
  async sendEvent(name: string, data: Record<string, any>) {
    try {
      const response = await POST({ name, data });
      console.log(response);
    } catch (err) {
      console.log(err);
    }
  },
  async configGet(propName: string): Promise<ApiResponse | undefined> {
    try {
      const response = await POST({ name: 'config.get', data: { propName } });
      return response;
    } catch (err) { }
    return undefined;
  },
  async configSet(propName: string, propValue: unknown): Promise<ApiResponse | undefined> {
    try {
      const response = await POST({ name: 'config.set', data: { propName, propValue } });
      return response;
    } catch (err) { }
    return undefined;
  }
}