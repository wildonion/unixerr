


# coding: utf-8

'''
	Codded By : 

 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██


'''

from cassandra.util import datetime_from_uuid1
from uuid import uuid1
from .db import init
from .db.schema import User, Position
from typing import Optional
from fastapi import FastAPI
from pydantic import BaseModel # you can use this with cassandra UserType in those cases that you don't have schemas!
import pandas as pd
import os



api = FastAPI()
db = None


@api.on_event("startup")
async def init_db():
	global db
	db = init()


@api.on_event("shutdown")
async def terminate_db():
	db.close()


@api.get("/")
async def welcome():
	return {"Welcome to": "uniXerr API Server"}


# #########------------------------------------------------------------------------------------------


@api.get("/users/info/{limit}")
async def get_users_info(limit: int):
	users_list_ = []
	users_list = []
	response = 200
	try: # for timeuuid ops refer to UUID and timeuuid functions on datastax docs
		future = db.query(f"select id, toTimestamp(time), rollcall_score, class_activity, discipline, total_quizzes_avg from users_info limit {limit};", [])
		users_ = future.result()
		for user_ in users_:
			users_list_.append(user_)
		users = User.objects().limit(limit)
		for user in users:
			user_dict = dict(user)
			user_dict["time"] = datetime_from_uuid1(user_dict["time"]) # convert uuid1 to datetime
			users_list.append(user_dict)
	except Exception as e:
		print(f"[Exception] ::: {e}")
		response = 500
	return {"db.query()_users": users_list_, "schema_users": users_list, "response": response}



# #########------------------------------------------------------------------------------------------


@api.get("/user/info/{user_id}") # ::: 'allow filtering' is only for development :::
async def get_user_info(user_id: int):
	response = 200
	try:
		future = db.query("select id, toTimestamp(time), rollcall_score, class_activity, discipline, total_quizzes_avg FROM users_info where id=? allow filtering", [user_id])
		user_ = future.result()[0]
		user = User.objects(id=user_id).allow_filtering()[0]
		user.time = datetime_from_uuid1(user.time)
	except Exception as e:
		print(f"[Exception] ::: {e}")
		response = 500
	return {"db.query()_user": user_, "schema_user": user, "response": response}


# #########------------------------------------------------------------------------------------------


@api.get("/users/positions/{limit}")
async def get_users_positions(limit: int):
	positions_list_ = []
	positions_list = []
	response = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions limit {limit};", [])
		positions_ = future.result()
		for position_ in positions_:
			positions_list_.append(position_)
		positions = Position.objects().limit(limit)
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"]) # convert uuid1 to datetime
			positions_list.append(position_dict)
	except Exception as e:
		response = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions": positions_list_, "schema_positions": positions_list, "response": response}



# #########------------------------------------------------------------------------------------------


@api.get("/users/position-latent/{position}") # ::: 'allow filtering' is only for development :::
async def get_users_position_latent(position: str):
	positions_latent_list_ = []
	positions_latent_list = []
	response = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where position_latent = ? allow filtering;", [position])
		positions_ = future.result()
		for position_ in positions_:
			positions_latent_list_.append(position_)
		positions = Position.objects.filter(position_latent=position).allow_filtering()
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"])
			positions_latent_list.append(position_dict)
	except Exception as e:
		response = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions_latent": positions_latent_list_, "schema_positions_latent": positions_latent_list, "response": response}



# #########------------------------------------------------------------------------------------------


@api.get("/users/position-raw/{position}") # ::: 'allow filtering' is only for development :::
async def get_users_position_raw(position: str):
	positions_raw_list_ = []
	positions_raw_list = []
	response = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where position_raw = ? allow filtering;", [position])
		positions_ = future.result()
		for position_ in positions_:
			positions_raw_list_.append(position_)
		positions = Position.objects.filter(position_raw=position).allow_filtering()
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"])
			positions_raw_list.append(position_dict)
	except Exception as e:
		response = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions_raw": positions_raw_list_, "schema_positions_raw": positions_raw_list, "response": response}



# #########------------------------------------------------------------------------------------------


@api.get("/users/positions/{latent}/{raw}") # ::: 'allow filtering' is only for development :::
async def get_users_same_positions(latent: str, raw: str):
	positions_LandR_list_ = []
	positions_LandR_list = []
	response = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where position_latent = ? and position_raw = ? allow filtering;", [latent, raw])
		positions_ = future.result()
		for position_ in positions_:
			positions_LandR_list_.append(position_)
		positions = Position.objects.filter(position_latent=latent).filter(position_raw=raw).allow_filtering()
		for position in positions:
			position_dict = dict(position)
			position_dict["time"] = datetime_from_uuid1(position_dict["time"])
			positions_LandR_list.append(position_dict)
	except Exception as e:
		response = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions_raw": positions_LandR_list_, "schema_positions_raw": positions_LandR_list, "response" : response}



# #########------------------------------------------------------------------------------------------


@api.get("/user/position/{user_id}") # ::: 'allow filtering' is only for development :::
async def get_user_position(user_id: int):
	response = 200
	try:
		future = db.query(f"select user_id, toTimestamp(time), position_latent, position_raw from users_positions where user_id = ? allow filtering;", [user_id])
		positions_ = future.result()[0]
		positions = Position.objects(user_id=user_id).allow_filtering()[0]
		positions.time = datetime_from_uuid1(positions.time)
	except Exception as e:
		response = 500
		print(f"[Exception] ::: {e}")
	return {"db.query()_positions": positions_, "schema_positions": positions, "response": response}



# #########------------------------------------------------------------------------------------------


@api.get("/users/add/info") # add rows of users from csv file into users_info table
async def add_users_info():
	futures = []
	response = 201
	input_data = os.path.dirname(os.path.abspath(__file__))+f'/dataset/input_data.csv'
	df = pd.read_csv(input_data)
	for i in range(len(df)):
		try:
			user = User(id=df.iloc[i].user_id, time=uuid1(), rollcall_score=df.iloc[i].rollcall_score, 
						class_activity=df.iloc[i].class_activity, discipline=df.iloc[i].discipline, 
						total_quizzes_avg=df.iloc[i].total_quizzes_avg)
			user.save()


			# #### --------------------------------------------------------------------------------
			# #### if you want to use db.query just comment User model to avoid duplicate insertion
			# #### --------------------------------------------------------------------------------

			# future = db.query("insert into users_info (id, time, rollcall_score, class_activity, discipline, total_quizzes_avg) values (?, ?, ?, ?, ?, ?)", 
			# 		  	  [df.iloc[i].user_id.astype('int'), uuid1(), df.iloc[i].rollcall_score.astype('int'), 
			# 		  	   df.iloc[i].class_activity, df.iloc[i].discipline, df.iloc[i].total_quizzes_avg
			# 		  	 ])
			# futures.append(future) # do what ever you want with futures like f.result()
		

		except Exception as e:
			print(f"[Exception] ::: {e}")
			response = 500
	return {"status": response}


# #########------------------------------------------------------------------------------------------



@api.get("/users/add/positions") # merge classified positions and then add those to positions table
async def add_users_positions():
	futures = []
	response = 201
	classified_latent = os.path.dirname(os.path.abspath(__file__))+f'/dataset/input_data_classified_positions_using-pre-trained_model_on-latent.csv'
	classified_raw = os.path.dirname(os.path.abspath(__file__))+f'/dataset/input_data_classified_positions_using-pre-trained_model_on-raw.csv'
	df_latent = pd.read_csv(classified_latent)
	df_raw = pd.read_csv(classified_raw)
	position_latent = df_latent["position"]
	position_raw = df_raw["position"]
	user_id = df_raw["user_id"]
	users_length = len(user_id)
	for i in range(users_length):
		try:
			user_position = Position(user_id=user_id.iloc[i], time=uuid1(), position_latent=position_latent.iloc[i], position_raw=position_raw.iloc[i])
			user_position.save()
			

			# #### ------------------------------------------------------------------------------------
			# #### if you want to use db.query just comment Position model to avoid duplicate insertion
			# #### ------------------------------------------------------------------------------------

			# future = db.query("insert into users_positions (user_id, time, position_latent, position_raw) values (?, ?, ?, ?)", 
			# 			  		[user_id.iloc[i], uuid1(), position_latent.iloc[i], position_raw.iloc[i]])
			# futures.append(future) # do what ever you want with futures like f.result()


		except Exception as e:
			print(f"Exception ::: {e}")
			response = 500
	return {"status": response}


# #########------------------------------------------------------------------------------------------