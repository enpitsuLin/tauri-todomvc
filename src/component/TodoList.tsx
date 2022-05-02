import { useAtom } from 'jotai'
import { v4 as randomUUID } from 'uuid'
import { useState, useCallback, KeyboardEventHandler } from 'react'
import { activeTodoCountAtom, allTodosAtom, anyTodosDone, filterType } from '../store/todos'
import { Todo } from '../types/todo'
import TodoItem from './TodoItem'
import { invoke } from '@tauri-apps/api'

const TodoList: React.FC<{ todos: Todo[] }> = ({ todos }) => {
  const [, setTodos] = useAtom(allTodosAtom)
  const [type, setType] = useAtom(filterType)
  const [activeCount] = useAtom(activeTodoCountAtom)
  const [anyDone] = useAtom(anyTodosDone)

  const [newTodo, setNewTodo] = useState('')

  const addTodo = async (label: string, id: string) => {
    invoke('new_todo', { todo: { id, label, done: false, is_delete: false } })
  }

  const onAddTodo = useCallback<KeyboardEventHandler<HTMLInputElement>>(
    (e) => {
      if (e.key === 'Enter') {
        e.preventDefault()
        if (newTodo) {
          const id = randomUUID()
          addTodo(newTodo, id)
          setTodos((oldTodos) => {
            return [...oldTodos, { label: newTodo, id, done: false } as Todo]
          })
          setNewTodo('')
        }
      }
    },
    [newTodo]
  )

  const onClearComplete = () => {
    setTodos((oldTodos) => {
      return oldTodos.filter((todo) => {
        const isDone = todo.done
        if (isDone) {
          invoke('update_todo', {
            todo: { ...todo, is_delete: true }
          })
          return false
        }
        return true
      })
    })
  }
  return (
    <>
      <header className="header">
        <h1>todos</h1>
        <input
          type="text"
          className="new-todo"
          value={newTodo}
          onChange={(e) => {
            setNewTodo(e.target.value)
          }}
          onKeyPress={onAddTodo}
          placeholder="What needs to be done?"
        />
      </header>
      <section className="main">
        <input type="checkbox" className="toggle-all" />
        <label htmlFor="togle-all"></label>
        <ul className="todo-list">
          {todos.map((todo) => (
            <TodoItem key={todo.id} todo={todo} />
          ))}
        </ul>
      </section>
      <footer className="footer">
        <span className="todo-count">
          <strong>{activeCount}</strong> items left
        </span>
        <ul className="filters">
          <li>
            <a onClick={() => setType('all')} className={type == 'all' ? 'selected' : ''}>
              All
            </a>
          </li>
          <li>
            <a onClick={() => setType('active')} className={type == 'active' ? 'selected' : ''}>
              Active
            </a>
          </li>
          <li>
            <a onClick={() => setType('completed')} className={type == 'completed' ? 'selected' : ''}>
              Completed
            </a>
          </li>
        </ul>
        {anyDone && (
          <button className="clear-completed" onClick={onClearComplete}>
            Clear completed
          </button>
        )}
      </footer>
    </>
  )
}
export default TodoList
