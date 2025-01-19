import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Register from "./pages/Register";
import Login from "./pages/Login";
import CreatePost from "./pages/CreatePost"; 
import PostsList from "./pages/PostsList";
import PostDetail from "./pages/PostDetail";
import UserPostsList from "./pages/UserPostsList"; 
import EditPost from "./pages/EditPost";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/register" element={<Register />} />
        <Route path="/login" element={<Login />} />
        <Route path="/create_post" element={<CreatePost />} /> 
        <Route path="/posts" element={<PostsList />} />
        <Route path="/posts/:id" element={<PostDetail />} />
        <Route path="/user_posts" element={<UserPostsList />} />
        <Route path="/posts/:id/edit" element={<EditPost />} />
      </Routes>
    </Router>
  );
}

export default App;