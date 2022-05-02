import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'react'
import TodoList from './component/TodoList'
import { Todo } from './types/todo'
function App() {
  const [todos, setTodos] = useState<Todo[]>([])
  useEffect(() => {
    invoke<Todo[]>('get_todos').then((res) => {
      setTodos(res)
    })
  }, [])
  return (
    <div className="todoapp">
      <TodoList todos={todos} />
    </div>
  )
}

export default App
