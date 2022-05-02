export function useDoubleClick(onClick: any, onDoubleClick: any) {
  let clicks: number[] = []
  let timeout: number

  return (event: any) => {
    clicks.push(new Date().getTime())

    clearTimeout(timeout)
    timeout = window.setTimeout(() => {
      if (clicks.length > 1 && clicks[clicks.length - 1] - clicks[clicks.length - 2] < 250) {
        if (onDoubleClick) {
          onDoubleClick(event)
        }
      } else if (onClick) {
        onClick(event)
      }
      clicks = []
    }, 250)
  }
}