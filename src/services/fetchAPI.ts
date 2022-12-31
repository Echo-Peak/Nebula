export const fetchAPI = {
  async sendEvent(name: string, data: Record<string, any>) {
    const response = await fetch('/api', {
      method: 'POST',
      body: JSON.stringify({
        name,
        data
      }),
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      }
    });
    console.log(response);
  }
}