import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Navbar from "../components/Navbar";

function EditPost() {
  const { id } = useParams(); 
  const navigate = useNavigate();
  const [formData, setFormData] = useState({
    title: "",
    content: "",
    published: false,
  });
  const [message, setMessage] = useState("");

  useEffect(() => {
    const fetchPost = async () => {
      try {
        const token = localStorage.getItem("token");
        if (!token) {
          setMessage("You must be logged in to edit a post.");
          navigate("/login");
          return;
        }

        const response = await fetch(`http://localhost:8000/posts/${id}/edit`, {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        const data = await response.json();
        if (data.success && data.post) {
          setFormData({
            title: data.post.title,
            content: data.post.content,
            published: data.post.published,
          });
        } else {
          setMessage("Failed to fetch post data.");
        }
      } catch (error) {
        console.error("Error fetching post data:", error);
        setMessage("An error occurred while fetching post data.");
      }
    };

    fetchPost();
  }, [id, navigate]);

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
        setMessage("You must be logged in to edit a post.");
        navigate("/login");
        return;
      }

      const response = await fetch(`http://localhost:8000/posts/${id}/update`, {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(formData),
      });

      const data = await response.json();
      if (data.success) {
        setMessage("Post updated successfully!");
        navigate("/user_posts"); 
      } else {
        setMessage(data.message || "Failed to update post.");
      }
    } catch (error) {
      console.error("Error updating post:", error);
      setMessage("An error occurred while updating the post.");
    }
  };

  return (
    <div>
      <Navbar />
      <div style={styles.content}>
        <h1>Edit Post</h1>
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
            Save Changes
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

export default EditPost;