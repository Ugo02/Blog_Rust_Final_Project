# Rust Project Application

This application was created for a Rust project. The backend of this application is built in Rust, the frontend is built in ReactJS, and the database is PostgreSQL.

## Backend Explanation

### 1. Technologies Used

- **Actix-web**: A powerful web framework for Rust that allows creating RESTful APIs with routes, middlewares, and request/response handling.
- **SQLx**: A library for executing SQL queries and mapping results into Rust objects.
- **Dotenv**: A tool to configure your application (e.g., database URL) without hardcoding values.
- **CORS (Cross-Origin Resource Sharing)**: Allows the frontend to communicate with the backend.

### 2. Backend Structure

- **`main.rs`**: This is the entry point of the application. It configures the HTTP server, routes, and the database connection.
- **`models/`**: Contains data structures (models) that represent the database tables.
- **`handlers/`**: Contains route handlers that manage the logic for each route.

### 3. Interaction with the Frontend

#### Routes
The backend exposes several routes to interact with the frontend:

- **Authentication**:
  - `POST /register`: User registration.
  - `POST /login`: User login.
  - `POST /logout`: User logout.

- **Post Management**:
  - `POST /create_post`: Create a new post.
  - `GET /posts`: Retrieve all posts.
  - `GET /posts/{id}`: Retrieve a specific post.
  - `GET /user_posts`: Retrieve posts for a specific user.
  - `GET /posts/{id}/edit`: Retrieve a post for editing.
  - `PUT /posts/{id}/update`: Update a post.

#### CORS
The CORS middleware is configured to allow requests from `http://localhost:3000` (the frontend). This enables the frontend to communicate with the backend without being blocked by browser security policies.

#### Database
The backend uses **SQLx** to interact with a PostgreSQL database. The database connection is managed via a connection pool (`PgPool`), which improves performance.

---

## Running the Application

To launch the application, simply navigate to the `frontend` directory and run the following command:

```bash
npm start