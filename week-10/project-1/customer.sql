--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text,
    c_email text,
    c_mobile character(15),
    eid integer,
    data_id integer NOT NULL,
    c_age integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_email, c_mobile, eid, data_id, c_age) FROM stdin;
110	Musta Karim	m_karim@gmail.com	8055089112     	102	5	35
111	Lilian Jaiya	l_jaiye@gmail.com	8055185431     	100	3	43
112	Arthur Musa	a_musa@gmail.com	7055282813     	107	10	50
113	Philip Akonjo	p_akonjo@gmail.com	8056765424     	100	2	41
114	Marylene Mapa	m_mapa@gmail.com	8053333551     	120	5	33
115	Oghenero Agor	o_agoro@gmail.com	7055566774     	117	11	50
116	Adams Bree	a_bree@gmail.com	8056765424     	102	1	33
117	Okafor Mathias	o_mathias@gmail.com	8056763367     	120	10	45
118	Samson Adeleke	s_adeleke@gmail.com	7056774423     	117	11	65
119	Lawal Tamire	l_tamire@gmail.com	9052111101     	107	5	35
120	James Job	j_job@gmail.com	8059693919     	100	8	44
121	Matthew Jakande	m_jakande@gmail.com	7051232144     	120	2	21
122	Jimila Adegboye	j_adegboye@gmail.com	8054921923     	107	5	20
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

