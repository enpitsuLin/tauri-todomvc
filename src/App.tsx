import { invoke } from '@tauri-apps/api'
import { useAtom } from 'jotai'
import { useEffect } from 'react'
import TodoList from './component/TodoList'
import { allTodosAtom, todosAtom } from './store/todos'
import { Todo } from './types/todo'

function App() {
  const [, setAllTodos] = useAtom(allTodosAtom)
  const [todos] = useAtom(todosAtom)
  useEffect(() => {
    invoke<Todo[]>('get_todos').then((res) => {
      setAllTodos(res)
    })
  }, [])
  return (
    <div className="todoapp">
      <TodoList todos={todos} />
    </div>
  )
}

export default App
