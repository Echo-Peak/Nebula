export const localStorageHelper = {
  get<T>(propName: string): T | undefined {
    const res = localStorage.getItem(propName);
    if (res) return JSON.parse(res) as T
  },
  set(propName: string, data: Record<string, any>) {
    localStorage.setItem(propName, JSON.stringify(data));
  }
}