export const resolveReader = async (
  reader: ReadableStreamDefaultReader<Uint8Array>,
  onData?: (text: string) => void,
  onEnd?: () => void
) => {
  let isEnd = false
  const result = []
  while (!isEnd) {
    const { done, value } = await reader.read()
    if (done) {
      isEnd = true
      onEnd?.()
    } else {
      const text = new TextDecoder().decode(value)
      result.push(text)
      onData?.(text)
    }
  }
  return result
}
