// App.jsx
import React from 'react';
import Navbar from './components/Navbar';
import Hero from './components/Hero';
import TechStack from './components/TechStack';
import Project from './components/Project';
import Contact from './components/Contact';
import './App.css';

function App() {
  return (
      <div className="App">
        <Navbar />
        <Hero />
        <TechStack />
        <Project title="Trainer's Corner" />
        <Project title="Provenance" />
        <Contact />
      </div>
  );
}

export default App;

// components/Navbar.jsx
import React, { useState } from 'react';
import './Navbar.css';

function Navbar() {
  const [isOpen, setIsOpen] = useState(false);

  return (
      <nav className="navbar fixed-top">
        <div className="nav-container">
          <button className="hamburger" onClick={() => setIsOpen(!isOpen)}>
            ☰
          </button>
          <div className={`nav-menu ${isOpen ? 'active' : ''}`}>
            <a href="#projects">Projects</a>
            <a href="#about">About</a>
            <a href="#links">Links</a>
          </div>
        </div>
      </nav>
  );
}

export default Navbar;

// components/Hero.jsx
import React from 'react';
import './Hero.css';

function Hero() {
  return (
      <section className="hero">
        <div className="profile-circle">
          {/* Replace with your image */}
          <img src="your-photo.jpg" alt="Timothy" />
        </div>
        <p className="intro-text">
          Hi, I'm Timothy, a software engineer out of Texas. I've been working on USPS and Medicare
          the last 3 years at Accenture and have two main personal projects: Trainer's Corner and Provenance.
        </p>
      </section>
  );
}

export default Hero;

// components/TechStack.jsx
import React from 'react';
import './TechStack.css';

function TechStack() {
  const tech = ['Java', 'Scala', 'Python', 'SQL', 'TypeScript', 'React', 'SpringBoot'];

  return (
      <section className="tech-stack">
        <h2>Technologies</h2>
        <div className="tech-list">
          {tech.map((item, index) => (
              <div key={index} className="tech-item">
                {item}
              </div>
          ))}
        </div>
      </section>
  );
}

export default TechStack;

// components/Project.jsx
import React from 'react';
import './Project.css';

function Project({ title }) {
  return (
      <section className="project">
        <h2>{title}</h2>
        <div className="project-content">
          {/* Add project-specific content here */}
          <p>Description of {title} goes here...</p>
        </div>
      </section>
  );
}

export default Project;

// components/Contact.jsx
import React from 'react';
import './Contact.css';

function Contact() {
  return (
      <section className="contact">
        <h2>Contact Me</h2>
        {/* Add contact form or info here */}
      </section>
  );
}

export default Contact;