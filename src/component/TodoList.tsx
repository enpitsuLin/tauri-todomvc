import { Todo } from '../types/todo'
import TodoItem from './TodoItem'

const TodoList: React.FC<{ todos: Todo[] }> = () => {
  return (
    <>
      <header className="header">
        <h1>todos</h1>
        <input type="text" className="new-todo" />
      </header>
      <section className="main">
        <input type="checkbox" className="toggle-all" />
        <label htmlFor="togle-all"></label>
        <ul className="todo-list">
          <TodoItem />
        </ul>
      </section>
      <footer className="footer">
        <span className="todo-count">
          <strong>1</strong> items left
        </span>
        <ul className="filters">
          <li>
            <a>All</a>
          </li>
          <li>
            <a>Active</a>
          </li>
          <li>
            <a>Completed</a>
          </li>
        </ul>
      </footer>
    </>
  )
}
export default TodoList
