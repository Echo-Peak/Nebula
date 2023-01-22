import { Backend } from './BackendAPI';


const format = (level: string, message: string) => {
  const date = new Date();
  return `${date.toLocaleDateString()}  ${level.toUpperCase()}: ${message}`
}

class LoggerAPI {
  public log(message: string, ...args: any[]) {
    const msg = format('info', message);
    if (!window.isDevelopment) return Backend.saveLog(msg, args);
    console.log(msg, ...args);
  }
  public warn(message: string, ...args: any[]) {
    const msg = format('warn', message);
    if (!window.isDevelopment) return Backend.saveLog(msg, args);
    console.log(msg, ...args);
  }
  public error(message: string, ...args: any[]) {
    const msg = format('error', message);
    if (!window.isDevelopment) return Backend.saveLog(msg, args);
    console.log(msg, ...args);
  }
  public debug(message: string, ...args: any[]) {
    const msg = format('debug', message);
    if (!window.isDevelopment) return;
    console.log(msg, ...args);
  }
}

export const logger = new LoggerAPI();