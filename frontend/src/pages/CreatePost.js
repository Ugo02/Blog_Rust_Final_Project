import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import Navbar from "../components/Navbar";

function CreatePost() {
  const [formData, setFormData] = useState({
    title: "",
    content: "",
    published: false,
  });
  const [message, setMessage] = useState("");
  const navigate = useNavigate();

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData({
      ...formData,
      [name]: type === "checkbox" ? checked : value,
    });
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const token = localStorage.getItem("token");
      if (!token) {
        setMessage("You must be logged in to create a post.");
        navigate("/login"); 
        return;
      }

      const response = await fetch("http://localhost:8000/create_post", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(formData),
      });

      const data = await response.json();
      if (data.success) {
        setMessage("Post created successfully!");
        navigate("/");
      } else {
        setMessage(data.message || "Failed to create post.");
      }
    } catch (error) {
      setMessage("An error occurred.");
    }
  };

  return (
    <div>
      <Navbar />
      <div style={styles.content}>
        <h1>Create a New Post</h1>
        <form onSubmit={handleSubmit} style={styles.form}>
          <div style={styles.formGroup}>
            <label>Title:</label>
            <input
              type="text"
              name="title"
              value={formData.title}
              onChange={handleChange}
              required
            />
          </div>
          <div style={styles.formGroup}>
            <label>Content:</label>
            <textarea
              name="content"
              value={formData.content}
              onChange={handleChange}
              required
              style={styles.textarea} 
            />
          </div>
          <div style={styles.formGroup}>
            <label>
              <input
                type="checkbox"
                name="published"
                checked={formData.published}
                onChange={handleChange}
              />
              Public
            </label>
          </div>
          <button type="submit" style={styles.submitButton}>
            Save Post
          </button>
        </form>
        <p>{message}</p>
      </div>
    </div>
  );
}

const styles = {
  content: {
    paddingTop: "80px",
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    textAlign: "center",
  },
  form: {
    display: "flex",
    flexDirection: "column",
    gap: "10px",
    width: "500px", 
  },
  formGroup: {
    display: "flex",
    flexDirection: "column",
    gap: "5px",
  },
  textarea: {
    height: "200px", 
    resize: "vertical", 
  },
  submitButton: {
    backgroundColor: "rgb(236, 92, 3)",
    color: "white",
    padding: "10px",
    border: "none",
    borderRadius: "5px",
    cursor: "pointer",
  },
};

export default CreatePost;