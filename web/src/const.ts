// export const SERVER_URL = 'http://101.126.78.130:9527'
export const SERVER_URL =
  process.env.NODE_ENV === 'production' ? '.' : 'http://127.0.0.1:9527'
