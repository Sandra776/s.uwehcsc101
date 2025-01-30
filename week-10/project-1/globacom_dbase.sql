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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text,
    data_duration_days integer,
    data_price_naira real
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character(1) NOT NULL,
    pduration text,
    project_managerid integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer,
    staff_sal real,
    age integer,
    mobile character(15)
);


ALTER TABLE public.staff OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration_days, data_price_naira) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I.	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	Alade joy	2	250000	33	08023089832    
100	Mustapha Ali	3	175000	32	08063285831    
107	Alokwe Martin	7	380000	48	07090082812    
97	Dankade Aminat	5	550000	40	09023688832    
108	Josiah Joshua	1	120000	30	08053189131    
102	Mankide Mary	2	450000	55	09023487830    
120	Adeleke Jane	4	200000	38	07061045862    
122	Osahon Mark	6	320000	44	08022289842    
117	Suleman Ajayi	3	800000	50	07030089811    
104	Kuti Lawal	1	750000	35	09145689842    
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dno);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- Name: staff staff_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT staff_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

